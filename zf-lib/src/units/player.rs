use gdnative::{api::PathFollow, prelude::*};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Player;

#[methods]
impl Player {
    fn new(_owner: &Node) -> Self {
        godot_print!("prepare Player");
        Player
    }

    #[export]
    fn _process(&self, owner: &Node, delta: f64) -> Option<()> {
        let follow = unsafe { owner.get_parent()?.assume_safe() }.cast::<PathFollow>()?;
        follow.set_unit_offset((follow.unit_offset() + 1. / 30. * delta).fract());
        Some(())
    }
}
