use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Hello;

#[methods]
impl Hello {
    fn new(_base: &Node) -> Self {
        Hello
    }

    #[method]
    fn _ready(&self) {
        godot_print!("hello, zf outside")
    }
}
