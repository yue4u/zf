use gdnative::{
    api::{object::ConnectFlags, Spatial},
    prelude::*,
};

use crate::{common::find_ref, entities::GameEvent, managers::VM, refs::groups, vm::VMSignal};

#[derive(NativeClass)]
#[inherit(Spatial)]
#[register_with(Self::register_signals)]
pub struct TutorialS1 {
    base: Ref<Spatial>,
}

const HIT_BY_PLAYER: &'static str = "hit_by_player";

#[methods]
impl TutorialS1 {
    fn new(base: TRef<Spatial>) -> Self {
        base.add_to_group(groups::ENEMY, false);
        TutorialS1 { base: base.claim() }
    }

    #[method]
    fn _ready(&self, #[base] base: TRef<Spatial>) -> Option<()> {
        let as_node = unsafe { base.get_node_as::<Node>(".")? };
        let vm_manager = find_ref::<VM, Node>(as_node)?;

        base.connect(
            HIT_BY_PLAYER,
            vm_manager,
            VMSignal::OnGameState.as_str(),
            VariantArray::new_shared(),
            ConnectFlags::DEFERRED.into(),
        )
        .expect("failed to connect hit_by_player");
        None
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal(HIT_BY_PLAYER).done();
    }

    #[method]
    pub fn damage(&self, #[base] base: TRef<Spatial>, _ammount: u32) {
        base.queue_free();

        unsafe { self.base.assume_safe() }
            .emit_signal(HIT_BY_PLAYER, &[GameEvent::EnemyDestroied.to_variant()]);
    }
}
