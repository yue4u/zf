use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Example;

#[methods]
impl Example {
    fn new(_base: &Node) -> Self {
        Example
    }

    #[method]
    fn _ready(&self) {
        tracing::info!("hello world")
    }
}
