use crate::vm::{CommandResult, VMConnecter, VMSignal};
use gdnative::{api::RichTextLabel, prelude::*};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct CommandResultDisplay;

#[methods]
impl CommandResultDisplay {
    fn new(_base: &Node) -> Self {
        CommandResultDisplay
    }

    #[method]
    fn _ready(&self, #[base] base: TRef<Node>) -> Option<()> {
        godot_print!("command result ready");
        base.connect_vm_signal(VMSignal::OnCmdResult.into());
        Some(())
    }

    #[method]
    fn on_cmd_result(&self, #[base] base: &Node, result: CommandResult) -> Option<()> {
        let result = match result.result {
            Ok(result) => result,
            Err(_) => format!("{:?}", result),
        };
        base.cast::<RichTextLabel>()?.set_bbcode(result);
        Some(())
    }
}
