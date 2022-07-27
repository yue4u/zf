use gdnative::{api::ItemList, prelude::*};

use crate::vm_connector;

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
        vm_connector::connect_on_cmd_entered(owner)
    }

    #[export]
    fn on_cmd_entered(&mut self, owner: &Node, text: String) -> Option<()> {
        godot_print!("add item {text}");
        owner
            .cast::<ItemList>()?
            .add_item(text, GodotObject::null(), false);
        Some(())
    }
}
