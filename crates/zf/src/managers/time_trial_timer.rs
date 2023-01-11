use gdnative::{api::object::ConnectFlags, prelude::*};

use crate::{common::find_ref, entities::GameEvent, managers::VM, vm::VMSignal};

#[derive(NativeClass)]
#[inherit(Timer)]
pub struct TimeTrialTimer;

#[methods]
impl TimeTrialTimer {
    fn new(_base: &Timer) -> Self {
        TimeTrialTimer
    }

    #[method]
    fn _ready(&self, #[base] base: TRef<Timer>) -> Option<()> {
        let as_node = unsafe { base.get_node_as::<Node>(".")? };
        let vm_manager = find_ref::<VM, Node>(as_node)?;
        base.connect(
            "timeout",
            vm_manager,
            VMSignal::OnGameState.as_str(),
            VariantArray::from_iter(&[GameEvent::MissionComplete("Time Trial Clear".to_owned())])
                .into_shared(),
            ConnectFlags::DEFERRED.into(),
        )
        .expect("failed to connect hit_by_player");
        Some(())
    }
}
