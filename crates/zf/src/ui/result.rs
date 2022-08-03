use crate::vm::{self, CommandResult};
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
        vm::connect_on_cmd_result(owner)
    }

    #[export]
    fn on_cmd_result(&mut self, owner: &Node, result: CommandResult) -> Option<()> {
        let result = format!("{:?}", result);
        owner.cast::<RichTextLabel>()?.set_bbcode(result);
        Some(())
    }
}
