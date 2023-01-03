use gdnative::{api::object::ConnectFlags, prelude::*};

use crate::{
    common::find_ref,
    entities::GameEvent,
    managers::VMManager,
    vm::{VMConnecter, VMSignal},
};

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct CommandInputWatcher {
    #[property]
    pub target_input: String,
}

const INPUT_MATCH: &'static str = "input_match";

#[methods]
impl CommandInputWatcher {
    fn new(_base: &Node) -> Self {
        CommandInputWatcher {
            target_input: "".to_owned(),
        }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal(INPUT_MATCH).done();
    }

    #[method]
    fn _ready(&self, #[base] base: TRef<Node>) -> Option<()> {
        tracing::trace!("watching for `{}`", self.target_input);
        let node = unsafe { base.get_node_as::<Node>(".")? };
        node.connect_vm_signal(VMSignal::OnCmdEntered.into());

        let vm_manager = find_ref::<VMManager, Node>(node)?;

        base.connect(
            INPUT_MATCH,
            vm_manager,
            VMSignal::OnGameState.as_str(),
            VariantArray::new_shared(),
            ConnectFlags::DEFERRED.into(),
        )
        .expect("failed to connect hit_by_player");

        Some(())
    }

    #[method]
    fn on_cmd_entered(&mut self, #[base] base: &Node, input: String) -> Option<()> {
        if input == self.target_input {
            base.emit_signal(
                INPUT_MATCH,
                &[GameEvent::MissionComplete("input ok".to_owned()).to_variant()],
            );
        }
        Some(())
    }
}
