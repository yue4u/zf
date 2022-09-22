use gdnative::{api::LineEdit, prelude::*};

#[derive(NativeClass)]
#[inherit(LineEdit)]
pub struct CommandPalette;

pub trait HandleCommandEntered {
    fn on_cmd_entered(&self, base: &LineEdit, text: String) -> Option<()>;
}

#[methods]
impl CommandPalette {
    fn new(_base: &LineEdit) -> Self {
        Self
    }

    #[method]
    fn _ready(&self, #[base] base: TRef<LineEdit>) -> Option<()> {
        godot_print!("line_edit ready");
        base.set_cursor_position(base.text().len() as i64);
        base.connect(
            "text_entered",
            base,
            "on_text_entered",
            VariantArray::new_shared(),
            0,
        )
        .expect("failed to connect line edit");
        base.grab_focus();

        Some(())
    }

    #[method]
    fn on_text_entered(&self, #[base] base: &LineEdit, _text: String) -> Option<()> {
        base.clear();
        Some(())
    }
}
