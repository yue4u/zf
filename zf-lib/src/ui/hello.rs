use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Hello;

#[methods]
impl Hello {
    pub(crate) fn new(_owner: &Node) -> Self {
        Hello
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("hello, zf outside")
    }
}
