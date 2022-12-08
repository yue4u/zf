use gdnative::{api::Engine, prelude::*};
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Display,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::JoinHandle,
    time::Duration,
};
use zf_ffi::{memory::Tag, CommandArgs, GameCommand, MissionCommand, TaskCommand};

use crate::{
    common::find_ref,
    entities::{GameState, Mission},
    refs::{groups, path::levels},
    ui::Terminal,
    vm::{CommandInput, CommandResult, VMSignal},
};

use zf_runtime::{decode_from_caller, Caller, ExtendedStore, Runtime};

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct VMManager {
    result_buffer: RefCell<ResultBuffer>,
    // TODO: more pts
    runtime: Option<Runtime<VMData>>,
}

type ResultBuffer = HashMap<u32, CommandResult>;

struct TaskRunner {
    id: usize,
    cmd: String,
    stop: Arc<AtomicBool>,
    handle: JoinHandle<()>,
}

impl TaskRunner {
    fn info(&self) -> TaskRunnerInfo {
        TaskRunnerInfo {
            id: self.id,
            cmd: self.cmd.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TaskRunnerInfo {
    id: usize,
    cmd: String,
}

#[derive(Serialize, Deserialize)]
struct TaskRunnerInfoVec(Vec<TaskRunnerInfo>);

impl Display for TaskRunner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("task: `{}` ({})", self.id, self.cmd))
    }
}

struct VMData {
    cmd_id: u32,
    thead_handles: HashMap<usize, TaskRunner>,
    base: Ref<Node>,
}

impl VMData {
    fn from_base(base: Ref<Node>) -> Self {
        Self {
            cmd_id: 0,
            thead_handles: HashMap::new(),
            base,
        }
    }
}

#[methods]
impl VMManager {
    pub(crate) fn new(_base: &Node) -> Self {
        VMManager {
            result_buffer: RefCell::new(HashMap::new()),
            runtime: None,
        }
    }

    pub(crate) fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal(VMSignal::OnCmdEntered.as_str()).done();
        builder.signal(VMSignal::OnCmdParsed.as_str()).done();
        builder.signal(VMSignal::OnCmdResult.as_str()).done();
        builder.signal(VMSignal::OnGameState.as_str()).done();
    }

    #[method]
    pub(crate) fn _ready(&mut self, #[base] base: TRef<Node>) {
        godot_print!("vm host ready");
        let mut runtime: Runtime<VMData> = VMData::from_base(base.claim()).into();
        runtime.eval(zf_runtime::SHELL_PRELOAD).unwrap();
        self.runtime = Some(runtime);
    }

    #[method]
    pub(crate) fn on_cmd_entered(&mut self, #[base] base: &Node, text: String) -> Option<()> {
        let runtime = self.runtime.as_mut()?;
        godot_print!("on_cmd_entered: {text}!");
        base.emit_signal(VMSignal::OnCmdEntered, &[Variant::new(text.clone())]);

        let result = runtime.eval(text).map_err(|e| e.to_string());
        let id = runtime.store.data_mut().ext.cmd_id + 1;
        runtime.store.data_mut().ext.cmd_id = id;
        let result = CommandResult { id, result };
        godot_dbg!(&result);
        base.emit_signal(VMSignal::OnCmdResult, &result.as_var());

        Some(())
    }

    #[method]
    pub fn on_cmd_result(&self, #[base] base: &Node, result: CommandResult) -> Option<()> {
        godot_print!("receive_command_result: {}", result.id);

        let mut result_buffer = self.result_buffer.borrow_mut();
        base.emit_signal(VMSignal::OnCmdResult, &result.as_var());
        result_buffer.insert(result.id, result);

        Some(())
    }

    #[method]
    pub fn on_game_state(&mut self, #[base] base: &Node, result: GameState) -> Option<()> {
        godot_print!("receive_game_state: {:?}", result);

        let runtime = self.runtime.as_mut()?;
        let result = runtime
            .eval("fsays 'mission completed'")
            .map_err(|e| e.to_string());
        let id = runtime.store.data_mut().ext.cmd_id + 1;
        runtime.store.data_mut().ext.cmd_id = id;
        let result = CommandResult { id, result };

        base.emit_signal(VMSignal::OnCmdResult, &result.as_var());
        unsafe { base.get_tree().unwrap().assume_safe() }.set_pause(true);
        Some(())
    }
}

fn fire_and_forget(vm_data: &VMData, cmd: CommandArgs) {
    godot_dbg!(&cmd);
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
        // terminal.state
        let size = terminal.map(|t, _| t.get_size()).expect("term.get_size");
        Tag::into(size.cols as i32, size.rows as i32)
    }

    fn zf_cmd(mut caller: Caller<'_, ExtendedStore<VMData>>, tag: i64) -> i64 {
        let cmd = decode_from_caller::<_, CommandArgs>(&mut caller, tag);
        godot_dbg!(&cmd);
        match cmd {
            CommandArgs::Task(task) => {
                let ret = match task {
                    TaskCommand::Run { cmd: input, every } => {
                        let store = caller.data_mut();
                        let base = store.ext.base;
                        let stop = Arc::new(AtomicBool::new(false));
                        let stop_clone = stop.clone();
                        let name = input.clone();
                        let handle = std::thread::spawn(move || {
                            let mut runtime: Runtime<VMData> = VMData::from_base(base).into();
                            let ret = runtime.eval(&input);
                            _ = godot_dbg!(ret);

                            if let Some(dur) = every {
                                loop {
                                    if stop_clone.load(Ordering::Relaxed) {
                                        godot_dbg!(format!("stop `{}` done!", &input));
                                        break;
                                    }
                                    std::thread::sleep(Duration::from_nanos(dur));
                                    let ret = runtime.eval(&input);
                                    _ = godot_dbg!(ret);
                                }
                            }
                        });
                        let task_id = caller.data_mut().ext.thead_handles.len() + 1;
                        let task = TaskRunner {
                            id: task_id,
                            stop,
                            cmd: name,
                            handle,
                        };
                        let start_info = format!("start {}", &task);
                        caller.data_mut().ext.thead_handles.insert(task_id, task);

                        start_info
                    }
                    TaskCommand::Stop(id) => (|| {
                        if let Ok(id) = id.parse() {
                            if let Some(task_runner) =
                                caller.data_mut().ext.thead_handles.remove(&id)
                            {
                                task_runner.stop.store(true, Ordering::Relaxed);
                                return format!("stop {}", task_runner);
                            };
                        }
                        format!("no task`{}` found", id)
                    })(),
                    TaskCommand::Status => {
                        let handles = &mut caller.data_mut().ext.thead_handles;

                        // clear finished task
                        handles.retain(|_, task_runner| !task_runner.handle.is_finished());

                        let info = handles
                            .values()
                            .map(|h| h.info())
                            .collect::<Vec<TaskRunnerInfo>>();
                        serde_json::to_string(&TaskRunnerInfoVec(info))
                            .expect("fail to serialize task runner info")
                    }
                };
                godot_dbg!(&ret);
                zf_runtime::write_string_with_caller(&mut caller, ret)
            }
            CommandArgs::Mission(m) => match m {
                MissionCommand::Info => {
                    zf_runtime::write_string_with_caller(&mut caller, Mission::dummy().summary())
                }
            },
            CommandArgs::Game(g) => {
                let tree = unsafe {
                    caller
                        .data()
                        .ext
                        .base
                        .assume_safe()
                        .get_tree()
                        .unwrap()
                        .assume_safe()
                };
                match g {
                    GameCommand::Start => {
                        // TODO: handle this error.
                        tree.change_scene(levels::SANDBOX).unwrap();
                        tree.set_pause(false);
                    }
                    GameCommand::Menu => {
                        tree.change_scene(levels::START_MENU).unwrap();
                        tree.set_pause(false);
                    }
                    GameCommand::Tutorial => {
                        tree.change_scene(levels::TUTORIAL_MOVEMENT).unwrap();
                        tree.set_pause(false);
                    }
                    GameCommand::End => {
                        tree.quit(0);
                    }
                };
                0
            }
            CommandArgs::Time(time) => {
                Engine::godot_singleton().set_time_scale(time.scale);
                0
            }
            CommandArgs::Radar(_) => {
                let radars = unsafe {
                    caller
                        .data()
                        .ext
                        .base
                        .assume_safe()
                        .get_tree()
                        .unwrap()
                        .assume_safe()
                }
                .get_nodes_in_group(groups::RADAR);
                // TODO: maybe more radars
                let result = unsafe {
                    radars
                        .get(0)
                        .call("detected", &[])
                        .unwrap()
                        .to::<String>()
                        .unwrap()
                };
                zf_runtime::write_string_with_caller(&mut caller, result)
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
