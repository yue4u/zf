use gdnative::{api::LineEdit, prelude::*};

use crate::vm;

#[derive(NativeClass)]
#[inherit(LineEdit)]
#[register_with(Self::register_signals)]
pub struct CommandPalette;

pub trait HandleCommandEntered {
    fn on_cmd_entered(&mut self, owner: &LineEdit, text: String) -> Option<()>;
}

#[methods]
impl CommandPalette {
    fn new(_owner: &LineEdit) -> Self {
        CommandPalette
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
            owner.emit_signal("on_cmd_parsed", &[Variant::new(command)]);
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
        unsafe { target.get_node(CommandPalette::path())?.assume_safe() }
            .cast::<LineEdit>()?
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
        unsafe { target.get_node(CommandPalette::path())?.assume_safe() }
            .cast::<LineEdit>()?
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
