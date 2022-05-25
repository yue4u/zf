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
    fn _ready(&self, owner: &Node) -> Option<()> {
        let follow = unsafe { owner.get_parent()?.assume_safe() }.cast::<PathFollow>()?;
        let tween = unsafe { owner.get_node("./Tween")?.assume_safe() }.cast::<Tween>()?;
        tween.interpolate_property(
            follow,
            "unit_offect",
            0,
            1,
            5000.,
            Tween::TRANS_LINEAR,
            0,
            0.,
        );
        Some(())
    }
}
