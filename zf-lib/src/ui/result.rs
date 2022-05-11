use crate::vm;
use gdnative::{
    api::{LineEdit, RichTextLabel},
    prelude::*,
};

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

        unsafe { owner.get_node("../CommandPalette/LineEdit")?.assume_safe() }
            .cast::<LineEdit>()?
            .connect(
                "text_entered",
                owner,
                "on_text_entered",
                VariantArray::new_shared(),
                0,
            )
            .expect("failed to connect line edit");
        Some(())
    }

    #[export]
    fn on_text_entered(&mut self, owner: &Node, text: String) -> Option<()> {
        let result = vm::exec(text);
        owner.cast::<RichTextLabel>()?.set_bbcode(result);
        Some(())
    }
}
