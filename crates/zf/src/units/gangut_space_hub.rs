use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GangutSpaceHub;

#[methods]
impl GangutSpaceHub {
    fn new(_base: &Node) -> Self {
        godot_print!("prepare GangutSpaceHub");
        GangutSpaceHub
    }

    #[method]
    fn _process(&self, #[base] base: &Node, delta: f64) -> Option<()> {
        base.cast::<Spatial>()?.rotate_y(delta * 0.1);
        Some(())
    }
}
