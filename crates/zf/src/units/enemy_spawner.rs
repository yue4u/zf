use gdnative::{
    api::{object::ConnectFlags, Area},
    prelude::*,
};
use zf_ffi::TaskListenableEvent;

use crate::{
    common::{find_ref, SceneLoader},
    managers::VM,
    refs::{self, groups},
    vm::VMSignal,
};
use rand::{rngs::ThreadRng, thread_rng, Rng};

use super::Player;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct EnemySpawner {
    #[property]
    pub max_spawn: u32,
    spawned: u32,
    player: Option<Instance<Player>>,
    rng: ThreadRng,
}

#[methods]
impl EnemySpawner {
    fn new(_base: &Node) -> Self {
        EnemySpawner {
            max_spawn: 1,
            spawned: 0,
            player: None,
            rng: thread_rng(),
        }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal(VMSignal::OnListenableEvent.as_str()).done();
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Node>) {
        let area = unsafe { base.get_node_as::<Area>("./Area").expect("area exist") };

        area.connect(
            "area_entered",
            base,
            "on_entered",
            VariantArray::new_shared(),
            0,
        )
        .expect("failed to connect area_entered");

        let vm_manager = find_ref::<VM, Node>(base).unwrap();

        base.connect(
            VMSignal::OnListenableEvent.as_str(),
            vm_manager,
            VMSignal::OnListenableEvent.as_str(),
            VariantArray::new_shared(),
            ConnectFlags::DEFERRED.into(),
        )
        .expect("failed to connect hit_by_player");

        let timer = unsafe { Timer::new().into_shared().assume_safe() };
        base.add_child(timer, false);
    }

    #[method]
    fn on_entered(&mut self, #[base] base: &Node, area: Ref<Area>) -> Option<()> {
        let parent = unsafe { area.assume_safe().get_parent()?.assume_safe() };
        if !parent.is_in_group(groups::PLAYER) {
            return None;
        }

        if self.spawned >= self.max_spawn {
            return None;
        }

        if self.player.is_none() {
            self.player = unsafe {
                base.get_node_as_instance::<Player>(Player::path_from(base))
                    .map(|player| player.claim())
            };
        }

        let player = unsafe { self.player.as_ref()?.base().assume_safe() };
        let enemy = SceneLoader::load_and_instance_as::<Spatial>(refs::path::scenes::ENEMY_S_1)?;
        let local = if self.rng.gen_bool(1. / 5.) {
            Vector3::new(0., 0., -40. + self.rng.gen_range(-5.0..=5.0))
        } else {
            Vector3::new(
                0. + self.rng.gen_range(-10.0..=10.0),
                5. + self.rng.gen_range(-10.0..=10.0),
                -40. + self.rng.gen_range(-5.0..=5.0),
            )
        };
        enemy.set_transform(Transform {
            basis: player.transform().basis,
            origin: local,
        });
        enemy.set_scale(Vector3::new(0.3, 0.3, 0.3));
        player.add_child(enemy, false);

        self.spawned += 1;

        // if parent
        base.emit_signal(
            VMSignal::OnListenableEvent,
            &[TaskListenableEvent::EnemyAppeared.to_variant()],
        );

        Some(())
    }
}
