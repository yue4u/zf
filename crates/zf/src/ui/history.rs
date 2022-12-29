use gdnative::{api::ItemList, prelude::*};

use crate::vm::{VMConnecter, VMSignal};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct CommandHistory;

#[methods]
impl CommandHistory {
    fn new(_base: &Node) -> Self {
        CommandHistory
    }

    #[method]
    fn _ready(&self, #[base] base: TRef<Node>) -> Option<()> {
        tracing::info!("command history ready");
        base.connect_vm_signal(VMSignal::OnCmdEntered.into());
        Some(())
    }

    #[method]
    fn on_cmd_entered(&self, #[base] base: &Node, text: String) -> Option<()> {
        tracing::info!("add item {text}");
        base.cast::<ItemList>()?
            .add_item(text, GodotObject::null(), false);
        Some(())
    }
}
