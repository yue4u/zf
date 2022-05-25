use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GangutSpaceHub;

#[methods]
impl GangutSpaceHub {
    fn new(_owner: &Node) -> Self {
        godot_print!("prepare GangutSpaceHub");
        GangutSpaceHub
    }

    #[export]
    fn _process(&self, owner: &Node, delta: f64) -> Option<()> {
        owner.cast::<Spatial>()?.rotate_y(delta * 0.1);
        Some(())
    }
}
