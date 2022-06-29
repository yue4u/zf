use std::collections::HashMap;

use gdnative::{api::Area, prelude::*};

use crate::units::player::Player;

#[derive(NativeClass, Default)]
#[inherit(Node)]
pub struct Radar {
    detected: HashMap<GodotString, Ref<Area>>,
}

#[methods]
impl Radar {
    fn new(_owner: &Node) -> Self {
        Default::default()
    }

    #[export]
    fn _ready(&self, owner: TRef<Node>) -> Option<()> {
        godot_print!("Radar ready");
        let player_radar = unsafe {
            owner
                .get_node(&format!("{}/{}", Player::path(), "RadarArea"))?
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
    fn _process(&mut self, owner: &Node, _delta: f64) -> Option<()> {
        let player = unsafe { owner.get_node(Player::path())?.assume_safe() }.cast::<Spatial>()?;

        self.detected.iter().for_each(|(id, area)| {
            let vec = unsafe { area.assume_safe() }.global_transform().origin
                - player.global_transform().origin;
            godot_print!("{}: {:?}", id, vec);
        });

        Some(())
    }

    #[export]
    fn on_detected(&mut self, _owner: &Node, area: Ref<Area>) -> Option<()> {
        let id = unsafe { area.assume_safe().get_parent()?.assume_safe() }.name();
        godot_print!("{}", id);
        self.detected.entry(id).or_insert(area);
        Some(())
    }

    #[export]
    fn on_lost(&mut self, _owner: &Node, _area: Ref<Area>) {
        godot_print!("on_lost")
    }
}
