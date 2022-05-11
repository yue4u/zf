use gdnative::{
    api::{ItemList, LineEdit},
    prelude::*,
};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct CommandHistory;

#[methods]
impl CommandHistory {
    fn new(_owner: &Node) -> Self {
        CommandHistory
    }

    #[export]
    fn _ready(&self, owner: TRef<Node>) -> Option<()> {
        godot_print!("command history ready");

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
        godot_print!("add item {text}");
        owner
            .cast::<ItemList>()?
            .add_item(text, GodotObject::null(), false);
        Some(())
    }
}
