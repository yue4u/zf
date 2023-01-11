use gdnative::{api::Engine, prelude::*};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, fmt::Display};
use zf_ffi::{
    memory::Tag, CommandArgs, CommandResults, GameCommand, LevelCommand, MissionCommand,
    ShieldCommand, TaskCommand, TaskListenableEvent,
};

use crate::{
    common::{current_level, find_ref, get_tree},
    entities::{GameEvent, LevelHelper, MissionLegacy, TargetPointInfo, LEVELS},
    refs::{
        groups, next_level,
        path::{auto_load, base_level, LevelName},
    },
    ui::{ScreenTransition, Terminal},
    units::Player,
    vm::{CommandInput, CommandResult, VMSignal},
};

use zf_runtime::{decode_from_caller, Caller, ExtendedStore, HostWrite, Runtime};

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct VM {
    cmds_available: Vec<String>,
    result_buffer: RefCell<ResultBuffer>,
    runtime: Option<Runtime<VMData>>,
    timer: Option<Ref<Timer>>,
}

type ResultBuffer = HashMap<u32, CommandResult>;

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: usize,
    cmd: String,
}

#[derive(Serialize, Deserialize)]
struct TaskList(Vec<Task>);

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("task: `{}` ({})", self.id, self.cmd))
    }
}

struct VMData {
    cmd_id: u32,
    background_tasks: HashMap<usize, Task>,
    listen_tasks: HashMap<TaskListenableEvent, String>,
    base: Ref<Node>,
}

impl VMData {
    fn from_base(base: Ref<Node>) -> Self {
        Self {
            cmd_id: 0,
            background_tasks: HashMap::new(),
            listen_tasks: HashMap::new(),
            base,
        }
    }

    fn scene_tree(&self) -> TRef<SceneTree> {
        unsafe { self.base.assume_safe().get_tree().unwrap().assume_safe() }
    }

    fn clean_background_tasks(&mut self) {
        tracing::debug!("vm: clean background_tasks");
        self.background_tasks.clear();
    }

    fn change_scene(&mut self, scene: &LevelName) {
        self.clean_background_tasks();

        unsafe {
            self.base
                .assume_safe()
                .get_node_as_instance::<ScreenTransition>(auto_load::POST_PROCESSING_TEXTURE_RECT)
        }
        .unwrap()
        .map_mut(|screen_transition, _| screen_transition.to(scene.to_owned()))
        .unwrap();
    }

    fn current_level(&self) -> LevelName {
        current_level(unsafe { self.base.assume_safe().as_ref() })
    }

    fn reload_scene(&mut self) {
        self.change_scene(&self.current_level());
    }

    fn target_point_info_in_group(&self, group: impl Into<GodotString>) -> Vec<TargetPointInfo> {
        self.scene_tree()
            .get_nodes_in_group(group)
            .iter()
            .filter_map(|point| point.to_object::<Spatial>())
            .map(|s| {
                let Vector3 { x, y, z } = unsafe { s.assume_safe() }.transform().origin;
                TargetPointInfo {
                    name: unsafe { s.assume_safe() }.name().to_string(),
                    pos: [x, y, z],
                }
            })
            .collect::<Vec<TargetPointInfo>>()
    }
}

#[methods]
impl VM {
    pub(crate) fn new(_base: &Node) -> Self {
        VM {
            cmds_available: vec![],
            result_buffer: RefCell::new(HashMap::new()),
            runtime: None,
            timer: None,
        }
    }

    pub(crate) fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal(VMSignal::OnCmdEntered.as_str()).done();
        builder.signal(VMSignal::OnCmdParsed.as_str()).done();
        builder.signal(VMSignal::OnCmdResult.as_str()).done();
        builder.signal(VMSignal::OnGameState.as_str()).done();
        builder.signal(VMSignal::OnListenableEvent.as_str()).done();
    }

    #[method]
    pub(crate) fn _ready(&mut self, #[base] base: TRef<Node>) {
        tracing::info!("vm host ready");
        let root = unsafe { base.get_node("/root").unwrap().assume_safe() };

        root.connect(
            "child_entered_tree",
            base,
            "on_child_entered_tree",
            VariantArray::new_shared(),
            0,
        )
        .expect("failed to connect child_entered_tree");

        let mut runtime: Runtime<VMData> = VMData::from_base(base.claim()).into();
        runtime.eval(zf_runtime::SHELL_PRELOAD).unwrap();

        // List all cmds available. we should not doing this here but
        // 1. it's too slow to eval this every time when recompiling
        // 2. it's too tedious to manually eval this every time when modify commands
        // 3. both term and vm must be ready if passing this info via signal
        if let Ok(cmds) = runtime.cmds_available() {
            self.cmds_available = cmds
        }

        self.runtime = Some(runtime);

        let timer = unsafe { Timer::new().into_shared().assume_safe() };
        base.add_child(timer, false);

        timer.set_one_shot(false);
        timer.start(-1.);
        timer
            .connect(
                "timeout",
                base,
                "trigger_background_tasks",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();
        self.timer = Some(timer.claim());
    }

    #[method]
    fn trigger_background_tasks(&mut self, #[base] _base: TRef<Node>) -> Option<()> {
        let runtime = self.runtime.as_mut()?;
        let cmds = runtime
            .store
            .data()
            .ext
            .background_tasks
            .values()
            .into_iter()
            .map(|t| t.cmd.clone())
            .collect::<Vec<String>>();

        for cmd in cmds {
            _ = runtime.eval(cmd);
        }
        Some(())
    }

    #[method]
    fn on_listenable_event(
        &mut self,
        #[base] _base: TRef<Node>,
        event: TaskListenableEvent,
    ) -> Option<()> {
        tracing::debug!("on_listenable_event: {}", event);
        let runtime = self.runtime.as_mut()?;
        let cmd = runtime
            .store
            .data()
            .ext
            .listen_tasks
            .get(&event)?
            .to_owned();
        _ = runtime.eval(cmd);
        Some(())
    }

    #[method]
    pub(crate) fn on_cmd_entered(&mut self, #[base] base: &Node, text: String) -> Option<()> {
        let runtime = self.runtime.as_mut()?;
        tracing::info!("on_cmd_entered: {text}!");
        base.emit_signal(VMSignal::OnCmdEntered, &[Variant::new(text.clone())]);

        let result = runtime.eval(text).map_err(|e| e.to_string());
        let id = runtime.store.data_mut().ext.cmd_id + 1;
        runtime.store.data_mut().ext.cmd_id = id;
        let result = CommandResult { id, result };
        tracing::debug!("{:?}", &result);
        base.emit_signal(VMSignal::OnCmdResult, &result.as_var());

        Some(())
    }

    #[method]
    pub fn on_cmd_result(&self, #[base] base: &Node, result: CommandResult) -> Option<()> {
        tracing::info!("receive_command_result: {}", result.id);

        let mut result_buffer = self.result_buffer.borrow_mut();
        base.emit_signal(VMSignal::OnCmdResult, &result.as_var());
        result_buffer.insert(result.id, result);

        Some(())
    }

    #[method]
    pub fn on_game_state(&mut self, #[base] base: &Node, event: GameEvent) -> Option<()> {
        tracing::trace!("receive_game_event: {:?}", event);

        let event = match event {
            GameEvent::MissionComplete(msg) => {
                let runtime = self.runtime.as_mut()?;
                runtime.store.data_mut().ext.clean_background_tasks();

                let result = runtime
                    .eval(format!("fsays 'Mission completed: {}'", msg))
                    .expect("fsays should work");
                unsafe {
                    base.get_node_as::<Label>(base_level::LEVEL_RESULT)
                        .unwrap()
                        .set_visible(true);
                }
                get_tree(base).set_pause(true);
                GameEvent::MissionComplete(result)
            }
            GameEvent::MissionFailed => {
                let runtime = self.runtime.as_mut()?;
                runtime.store.data_mut().ext.clean_background_tasks();
                // let result = runtime
                //     .eval(format!("fsays 'Mission completed: {}'", "Mission failed"))
                //     .expect("fsays should work");
                let label = unsafe { base.get_node_as::<Label>(base_level::LEVEL_RESULT).unwrap() };
                label.set_text("Mission failed");
                // cd00fff3 -> cdff0099
                // a1e4c95f -> a17052b0
                label.add_color_override("font_color", Color::from_html("#cdff0099").unwrap());
                label.add_color_override(
                    "font_color_shadow",
                    Color::from_html("#a17052b0").unwrap(),
                );
                label.set_visible(true);
                get_tree(base).set_pause(true);
                GameEvent::MissionFailed
            }
            as_is => as_is,
        };

        base.emit_signal(VMSignal::OnGameState, &[event.to_variant()]);
        Some(())
    }

    #[method]
    fn on_child_entered_tree(&mut self, #[base] base: &Node, node: Ref<Node>) {
        let scene = current_level(unsafe { node.assume_safe() }.as_ref());
        base.emit_signal(
            VMSignal::OnGameState,
            &[GameEvent::LevelChange(scene).to_variant()],
        );
    }

    pub fn complete(&self, buffer: &str) -> Vec<String> {
        self.cmds_available
            .iter()
            .filter(|cmd| cmd.starts_with(buffer) && cmd.len() != buffer.len())
            .take(5)
            .map(|cmd| cmd.to_owned())
            .collect::<Vec<String>>()
    }
}

fn fire_and_forget(vm_data: &VMData, cmd: CommandArgs) {
    tracing::debug!("{:?}", &cmd);
    unsafe { vm_data.base.assume_safe() }.emit_signal(
        VMSignal::OnCmdParsed,
        &[CommandInput {
            id: vm_data.cmd_id,
            cmd,
        }
        .to_variant()],
    );
}

struct RuntimeFunc;

impl RuntimeFunc {
    fn zf_terminal_size(caller: Caller<'_, ExtendedStore<VMData>>) -> i64 {
        let terminal =
            find_ref::<Terminal, Control>(unsafe { caller.data().ext.base.assume_safe() })
                .expect("find ref termial")
                .cast_instance::<Terminal>()
                .expect("cast instance termial");
        let size = terminal.map(|t, _| t.get_size()).expect("term.get_size");
        Tag::into(size.cols as i32, size.rows as i32)
    }

    fn zf_cmd(mut caller: Caller<'_, ExtendedStore<VMData>>, tag: i64) -> i64 {
        let cmd = decode_from_caller::<_, CommandArgs>(&mut caller, tag);
        tracing::debug!("{:?}", &cmd);
        match cmd {
            CommandArgs::Task(task) => {
                let ret = match task {
                    TaskCommand::Run { cmd } => {
                        let task_id = caller.data_mut().ext.background_tasks.len() + 1;
                        let task = Task { id: task_id, cmd };
                        let start_info = format!("start {}", &task);
                        caller.data_mut().ext.background_tasks.insert(task_id, task);

                        start_info
                    }
                    TaskCommand::On { event, cmd } => {
                        let start_info = format!("start {cmd} on {event}");
                        caller.data_mut().ext.listen_tasks.insert(event, cmd);

                        start_info
                    }
                    TaskCommand::Stop(id) => (|| {
                        if let Ok(id) = id.parse() {
                            if let Some(task_runner) =
                                caller.data_mut().ext.background_tasks.remove(&id)
                            {
                                return format!("stop {}", task_runner);
                            };
                        }
                        format!("no task`{}` found", id)
                    })(),
                    TaskCommand::Status => {
                        let handles = &mut caller.data_mut().ext.background_tasks;

                        let info = handles.values().map(|t| t.clone()).collect::<Vec<Task>>();
                        serde_json::to_string(&info).expect("fail to serialize task runner info")
                    }
                };
                tracing::debug!("{:?}", &ret);
                caller.write_string_from_host(ret)
            }
            CommandArgs::Mission(m) => match m {
                MissionCommand::Info => {
                    // FIXME: this is outdated
                    caller.write_string_from_host(MissionLegacy::dummy().summary())
                }
                MissionCommand::Targets => {
                    let targets = caller
                        .data()
                        .ext
                        .target_point_info_in_group(groups::TARGET_POINT);
                    caller.write_json(&targets)
                }
            },
            CommandArgs::Game(g) => {
                match g {
                    GameCommand::Start => {
                        caller
                            .data_mut()
                            .ext
                            .change_scene(&LevelName::ChallengeInfinite);
                    }
                    GameCommand::Menu => {
                        caller.data_mut().ext.change_scene(&LevelName::StartMenu);
                    }
                    GameCommand::End => {
                        caller.data().ext.scene_tree().quit(0);
                    }
                };
                0
            }
            CommandArgs::Level(level) => match level {
                LevelCommand::Start(name) => {
                    let scene = LevelName::from(&name);
                    if scene != LevelName::Unknown {
                        caller.data_mut().ext.change_scene(&scene);
                    }
                    0
                }
                LevelCommand::Restart => {
                    caller.data_mut().ext.reload_scene();
                    0
                }
                LevelCommand::Next => {
                    let current = caller.data().ext.current_level().to_string();
                    if let Some(next) = next_level(current) {
                        caller.data_mut().ext.change_scene(next);
                    };
                    0
                }
                LevelCommand::List => {
                    let levels = LEVELS
                        .iter()
                        .map(|l| l.to_string())
                        .collect::<Vec<String>>();
                    caller.write_result(CommandResults::Levels(levels))
                }
            },
            CommandArgs::Time(time) => {
                Engine::godot_singleton().set_time_scale(time.scale);
                0
            }
            CommandArgs::Tutorial => {
                caller
                    .data_mut()
                    .ext
                    .change_scene(&LevelName::TutorialHelloWorld);
                0
            }
            CommandArgs::Hint => {
                let hint = caller.data().ext.current_level().hint();
                caller.write_string_from_host(hint)
            }
            CommandArgs::Radar(_) => {
                let targets = caller.data().ext.target_point_info_in_group(groups::ENEMY);
                caller.write_json(&targets)
            }
            CommandArgs::Shield(ShieldCommand::Show) => {
                let base = unsafe { caller.data().ext.base.assume_safe() };

                unsafe { base.get_node_as_instance::<Player>(Player::path_from(base.as_ref())) }
                    .and_then(|instance| {
                        instance
                            .map(|player, _| caller.write_json(player.shield()))
                            .ok()
                    })
                    .unwrap_or(0)
            }
            cmd => {
                fire_and_forget(&mut caller.data_mut().ext, cmd);
                0
            }
        }
    }
}

impl Into<Runtime<VMData>> for VMData {
    fn into(self) -> Runtime<VMData> {
        Runtime::init(self, |linker| -> anyhow::Result<()> {
            linker
                .func_wrap("zf", "zf_terminal_size", RuntimeFunc::zf_terminal_size)?
                .func_wrap("zf", "zf_cmd", RuntimeFunc::zf_cmd)?;

            Ok(())
        })
        .expect("failed to init runtime")
    }
}
