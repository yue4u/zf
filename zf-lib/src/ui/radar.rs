use gdnative::{api::Area, prelude::*};

use crate::units;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Radar;

#[methods]
impl Radar {
    fn new(_owner: &Node) -> Self {
        Radar
    }

    #[export]
    fn _ready(&self, owner: TRef<Node>) -> Option<()> {
        godot_print!("Radar ready");
        let player_radar = unsafe {
            owner
                .get_node(&format!("{}/{}", units::Player::path(), "RadarArea"))?
                .assume_safe()
        }
        .cast::<Area>()?;

        godot_dbg!(player_radar);

        player_radar
            .connect(
                "area_entered",
                owner,
                "on_detected",
                VariantArray::new_shared(),
                0,
            )
            .expect("failed to connect area_entered");

        player_radar
            .connect(
                "area_exited",
                owner,
                "on_lost",
                VariantArray::new_shared(),
                0,
            )
            .expect("failed to connect area_exited");

        Some(())
    }

    #[export]
    fn on_detected(&mut self, _owner: &Node) {
        godot_print!("on_detected")
    }

    #[export]
    fn on_lost(&mut self, _owner: &Node) {
        godot_print!("on_lost")
    }
}
