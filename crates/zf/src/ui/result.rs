use crate::vm::{CommandResult, VMConnecter, VMSignal};
use gdnative::{api::RichTextLabel, prelude::*};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct CommandResultDisplay;

#[methods]
impl CommandResultDisplay {
    fn new(_owner: &Node) -> Self {
        CommandResultDisplay
    }

    #[export]
    fn _ready(&self, owner: TRef<Node>) -> Option<()> {
        godot_print!("command result ready");
        owner.connect_vm_signal(VMSignal::OnCmdResult);
        Some(())
    }

    #[export]
    fn on_cmd_result(&self, owner: &Node, result: CommandResult) -> Option<()> {
        let result = match result.result {
            Ok(result) => result,
            Err(_) => format!("{:?}", result),
        };
        owner.cast::<RichTextLabel>()?.set_bbcode(result);
        Some(())
    }
}
