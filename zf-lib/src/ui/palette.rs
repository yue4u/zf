use gdnative::{api::LineEdit, prelude::*};

use crate::{bind_path, common::HasPath};

#[derive(NativeClass)]
#[inherit(LineEdit)]
pub struct CommandPalette;

pub trait HandleCommandEntered {
    fn on_cmd_entered(&mut self, owner: &LineEdit, text: String) -> Option<()>;
}

#[methods]
impl CommandPalette {
    fn new(_owner: &LineEdit) -> Self {
        Self
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
    fn on_text_entered(&mut self, owner: &LineEdit, _text: String) -> Option<()> {
        owner.clear();
        Some(())
    }
}

bind_path!(CommandPalette, space::LINE_EDIT);
