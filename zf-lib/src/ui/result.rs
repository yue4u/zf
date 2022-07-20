use crate::{
    ui::CommandPalette,
    vm::{self, Command},
};
use gdnative::{api::RichTextLabel, prelude::*};

use super::CommandInput;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct CommandResult;

#[methods]
impl CommandResult {
    fn new(_owner: &Node) -> Self {
        CommandResult
    }

    #[export]
    fn _ready(&self, owner: TRef<Node>) -> Option<()> {
        godot_print!("command result ready");
        CommandPalette::connect_on_cmd_parsed(owner)
    }

    #[export]
    fn on_cmd_parsed(&mut self, owner: &Node, input: CommandInput) -> Option<()> {
        let result = vm::exec(input.cmd);
        owner.cast::<RichTextLabel>()?.set_bbcode(result);
        Some(())
    }
}
