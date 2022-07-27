use std::cell::RefCell;

use super::CommandInput;
use super::CommandResult;
use crate::common::find_ref;
use crate::common::HasPath;
use crate::ui::CommandPalette;
use crate::vm::Parser;
use gdnative::prelude::*;

#[derive(NativeClass, Debug)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct VMHost {
    pub index: RefCell<u32>,
}

#[methods]
impl VMHost {
    pub(crate) fn new(_owner: &Node) -> Self {
        VMHost {
            index: RefCell::new(0),
        }
    }

    pub(crate) fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal("on_cmd_entered").done();
        builder.signal("on_cmd_parsed").done();
    }

    #[export]
    pub(crate) fn _ready(&self, owner: TRef<Node>) -> Option<()> {
        godot_print!("vm host ready");
        find_ref::<CommandPalette, Node>(owner)?
            .connect(
                "text_entered",
                owner,
                "on_cmd_entered",
                VariantArray::new_shared(),
                0,
            )
            .expect("failed to connect line edit");

        Some(())
    }

    #[export]
    pub(crate) fn on_cmd_entered(&self, owner: &Node, text: String) -> Option<()> {
        godot_print!("on_cmd_entered: {text}!");
        owner.emit_signal("on_cmd_entered", &[Variant::new(text.clone())]);

        if let Ok(command_run) = Parser::parse(text) {
            for command in command_run.cmds {
                let index = self.index.replace_with(|&mut i| i + 1);
                let command_input = CommandInput {
                    cmd: command,
                    index,
                };
                godot_print!("command: {:?}!", &command_input);
                owner.emit_signal("on_cmd_parsed", &[Variant::new(command_input)]);
            }
        }
        Some(())
    }

    pub fn receive_command_result(&self, result: CommandResult) -> Option<()> {
        godot_print!("receive_command_result: {:?}!", result);
        Some(())
    }
}

impl HasPath for VMHost {
    fn path() -> &'static str {
        "/root/Scene/Managers/VMHost"
    }
}
