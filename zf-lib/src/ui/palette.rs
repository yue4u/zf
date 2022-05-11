use gdnative::{api::LineEdit, prelude::*};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct CommandPalette;

#[methods]
impl CommandPalette {
    fn new(_owner: &Node) -> Self {
        CommandPalette
    }

    #[export]
    fn _ready(&self, owner: TRef<Node>) -> Option<()> {
        godot_print!("line_edit ready");
        let line_edit = owner.cast::<LineEdit>()?;

        line_edit
            .connect(
                "text_entered",
                owner,
                "on_text_entered",
                VariantArray::new_shared(),
                0,
            )
            .expect("failed to connect line edit");
        line_edit.grab_focus();

        Some(())
    }

    #[export]
    fn on_text_entered(&mut self, owner: &Node, text: String) -> Option<()> {
        godot_print!("on_text_entered: {text}!");
        owner.cast::<LineEdit>()?.clear();
        Some(())
    }
}
