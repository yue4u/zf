use gdnative::{
    api::{object::ConnectFlags, TextureProgress},
    prelude::*,
};

use crate::{
    common::{find_ref, LookAtPlauer},
    entities::GameEvent,
    managers::VM,
    refs::{groups, path},
    vm::VMSignal,
    weapons::Launcher,
};

#[derive(NativeClass)]
#[inherit(Spatial)]
#[register_with(Self::register_signals)]
pub struct EnemyS1 {
    hp: Option<Ref<TextureProgress>>,
    base_ref: Ref<Spatial>,
}

const ON_DESTROY: &'static str = "ON_DESTROY";

#[methods]
impl EnemyS1 {
    fn new(base: TRef<Spatial>) -> Self {
        base.add_to_group(groups::ENEMY, false);
        Launcher::load_with_weapon(base, path::scenes::BEAM);

        EnemyS1 {
            hp: None,
            base_ref: base.claim(),
        }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal(ON_DESTROY).done();
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Spatial>) -> Option<()> {
        let hp = unsafe {
            base.get_node("HP/Viewport/Control")
                .expect("expect TextureProgress")
                .assume_safe()
        }
        .cast::<TextureProgress>()
        .expect("expect can cast TextureProgress")
        .claim();
        self.hp = Some(hp);

        let as_node = unsafe { base.get_node_as::<Node>(".")? };
        let vm_manager = find_ref::<VM, Node>(as_node)?;

        base.connect(
            ON_DESTROY,
            vm_manager,
            VMSignal::OnGameState.as_str(),
            VariantArray::new_shared(),
            ConnectFlags::DEFERRED.into(),
        )
        .expect("failed to connect hit_by_player");

        Some(())
    }

    #[method]
    fn _process(&self, #[base] base: TRef<Spatial>, _delta: f64) -> Option<()> {
        base.try_look_at_player()
    }

    #[method]
    pub fn damage(&self, #[base] base: TRef<Spatial>, _ammount: u32) {
        let hp = unsafe { self.hp.unwrap().assume_safe() };
        let hp_tmp = hp.value() - 0.5 * hp.max();
        hp.set_value(hp_tmp);
        if hp_tmp < 0. {
            base.emit_signal(ON_DESTROY, &[GameEvent::EnemyDestroied.to_variant()]);
            unsafe { self.base_ref.assume_safe() }.queue_free()
        }
    }
}
