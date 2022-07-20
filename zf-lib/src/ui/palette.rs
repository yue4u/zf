use gdnative::{api::LineEdit, prelude::*};

use crate::vm::{self, Command};

#[derive(NativeClass)]
#[inherit(LineEdit)]
#[register_with(Self::register_signals)]
pub struct CommandPalette {
    pub index: u32,
}

#[derive(FromVariant, ToVariant)]
pub struct CommandInput {
    pub cmd: Command,
    pub index: u32,
}

pub trait HandleCommandEntered {
    fn on_cmd_entered(&mut self, owner: &LineEdit, text: String) -> Option<()>;
}

#[methods]
impl CommandPalette {
    fn new(_owner: &LineEdit) -> Self {
        CommandPalette { index: 0 }
    }

    #[export]
    fn _ready(&self, owner: TRef<LineEdit>) -> Option<()> {
        godot_print!("line_edit ready");
        owner.set_cursor_position(owner.text().len() as i64);
        owner
            .connect(
                "text_entered",
                owner,
                "on_text_entered",
                VariantArray::new_shared(),
                0,
            )
            .expect("failed to connect line edit");
        owner.grab_focus();

        Some(())
    }

    #[export]
    fn on_text_entered(&mut self, owner: &LineEdit, text: String) -> Option<()> {
        godot_print!("on_text_entered: {text}!");
        owner.cast::<LineEdit>()?.clear();

        if let Ok(command) = vm::Parser::parse(text) {
            let command_input = CommandInput {
                cmd: command,
                index: self.index,
            };
            self.index += 1;
            owner.emit_signal("on_cmd_parsed", &[Variant::new(command_input)]);
        }

        Some(())
    }

    pub fn path() -> &'static str {
        "/root/Scene/UI/MarginContainer/Control/CommandPalette/LineEdit"
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal("on_cmd_parsed").done();
    }

    pub fn connect_on_cmd_entered(target: TRef<Node>) -> Option<()> {
        find_line_edit(target)?
            .connect(
                "text_entered",
                target,
                "on_cmd_entered",
                VariantArray::new_shared(),
                0,
            )
            .expect("failed to connect line edit");
        Some(())
    }

    pub fn connect_on_cmd_parsed(target: TRef<Node>) -> Option<()> {
        find_line_edit(target)?
            .connect(
                "on_cmd_parsed",
                target,
                "on_cmd_parsed",
                VariantArray::new_shared(),
                0,
            )
            .expect("failed to connect line edit on_cmd_parsed");
        Some(())
    }

    pub fn connect_on_cmd_result(target: TRef<Node>) -> Option<()> {
        find_line_edit(target)?
            .connect(
                "on_cmd_parsed",
                target,
                "on_cmd_parsed",
                VariantArray::new_shared(),
                0,
            )
            .expect("failed to connect line edit on_cmd_parsed");
        Some(())
    }
}

fn find_line_edit(target: TRef<Node>) -> Option<TRef<LineEdit>> {
    unsafe { target.get_node(CommandPalette::path())?.assume_safe() }.cast::<LineEdit>()
}
