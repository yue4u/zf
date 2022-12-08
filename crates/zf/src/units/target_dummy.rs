use gdnative::{
    api::{object::ConnectFlags, Spatial},
    prelude::*,
};

use crate::{
    common::find_ref, entities::GameState, managers::VMManager, refs::groups, vm::VMSignal,
};

#[derive(NativeClass)]
#[inherit(Spatial)]
#[register_with(Self::register_signals)]
pub struct TargetDummy {
    base: Ref<Spatial>,
}

const HIT_BY_PLAYER: &'static str = "hit_by_player";

#[methods]
impl TargetDummy {
    fn new(base: TRef<Spatial>) -> Self {
        base.add_to_group(groups::ENEMY, false);
        TargetDummy { base: base.claim() }
    }

    #[method]
    fn _ready(&self, #[base] base: TRef<Spatial>) -> Option<()> {
        let as_node = unsafe { base.get_node_as::<Node>(".")? };
        let vm_manager = find_ref::<VMManager, Node>(as_node)?;

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
    pub fn damage(&self) {
        unsafe { self.base.assume_safe() }
            .emit_signal(HIT_BY_PLAYER, &[GameState::MissionComplete.to_variant()]);
    }
}
