use crate::{
    common::{current_scene, find_ref, get_tree},
    entities::GameEvent,
    managers::VMManager,
    refs::{groups, path::SceneName},
    vm::VMSignal,
};
use gdnative::{
    api::{object::ConnectFlags, RichTextLabel},
    prelude::*,
};

#[derive(NativeClass)]
#[inherit(RichTextLabel)]
#[register_with(Self::register_signal)]
pub struct Mission {
    base: Ref<RichTextLabel>,
    mission: Option<MissionDetails>,
}

struct MissionDetails {
    scene: SceneName,
    target_points: u32,
    target_points_all: u32,
    enemies: u32,
    enemies_all: u32,
    // TODO: cmds
    // cmds: u32,
}

const ON_MISSION_STATE: &'static str = "ON_MISSION_STATE";

#[methods]
impl Mission {
    fn new(base: TRef<RichTextLabel>) -> Self {
        Mission {
            base: base.claim(),
            mission: None,
        }
    }

    pub fn register_signal<T: NativeClass>(builder: &ClassBuilder<T>) {
        builder.signal(ON_MISSION_STATE).done();
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<RichTextLabel>) -> Option<()> {
        let as_node = unsafe { base.get_node_as::<Node>(".")? };
        let vm_manager = find_ref::<VMManager, Node>(as_node)?;

        vm_manager
            .connect(
                VMSignal::OnGameState,
                base,
                VMSignal::OnGameState,
                VariantArray::new_shared(),
                ConnectFlags::DEFERRED.into(),
            )
            .expect("failed to connect vm");

        base.connect(
            ON_MISSION_STATE,
            vm_manager,
            VMSignal::OnGameState,
            VariantArray::new_shared(),
            ConnectFlags::DEFERRED.into(),
        )
        .expect("failed to connect vm");

        self.init_mission();
        Some(())
    }

    #[method]
    fn on_game_state(
        &mut self,
        #[base] _base: TRef<RichTextLabel>,
        state: GameEvent,
    ) -> Option<()> {
        let m = self.mission.as_mut().unwrap();
        match state {
            // GameEvent::MissionComplete(msg) => {}
            GameEvent::LevelChange(scene) => {
                m.scene = scene;
                self.update_text();
            }
            GameEvent::HitTargetPoint => {
                m.target_points += 1;
                self.update_text();
            }
            GameEvent::EnemyDestroied => {
                m.enemies += 1;
                self.update_text();
            }
            _ => {}
        };
        Some(())
    }

    fn init_mission(&mut self) {
        let as_node_ref = unsafe { self.base.assume_safe().get_node_as::<Node>(".") }
            .unwrap()
            .as_ref();
        let tree = get_tree(as_node_ref);
        let m = MissionDetails {
            scene: current_scene(as_node_ref),
            target_points: 0,
            target_points_all: tree.get_nodes_in_group(groups::TARGET_POINT).len() as u32,
            enemies: 0,
            enemies_all: tree.get_nodes_in_group(groups::ENEMY).len() as u32,
        };
        self.mission = Some(m);
        self.update_text();
    }

    fn update_text(&self) {
        let MissionDetails {
            scene,
            target_points,
            target_points_all,
            enemies,
            enemies_all,
        } = self.mission.as_ref().unwrap();
        let text = format!(
            r#"[b][color=#4FFFCA]Mission: {scene}[/color][/b]
Target points: {target_points} / {target_points_all}
Target enemies: {enemies} / {enemies_all}
"#
        );
        let base = unsafe { self.base.assume_safe() };
        base.set_bbcode(text);

        if enemies == enemies_all && target_points == target_points_all {
            base.emit_signal(
                ON_MISSION_STATE,
                &[GameEvent::MissionComplete(scene.to_string()).to_variant()],
            );
        }
    }
}
