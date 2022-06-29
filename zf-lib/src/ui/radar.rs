use std::collections::HashMap;

use gdnative::{
    api::{Area, TextureRect},
    prelude::*,
};

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
        for entry in self.detected.iter() {
            render_marker(owner, player, entry);
        }
        Some(())
    }

    #[export]
    fn on_detected(&mut self, owner: &Node, area: Ref<Area>) -> Option<()> {
        let id = unsafe { area.assume_safe().get_parent()?.assume_safe() }.name();
        self.detected.insert(id.clone(), area);
        let enemy = unsafe {
            owner
                .get_node("pawn")?
                .assume_safe()
                .duplicate(0)?
                .assume_safe()
        };
        enemy.set_name(id);
        enemy.set("visible", true);
        owner.add_child(enemy, false);
        Some(())
    }

    #[export]
    fn on_lost(&mut self, owner: &Node, area: Ref<Area>) -> Option<()> {
        let id = unsafe { area.assume_safe().get_parent()?.assume_safe() }.name();
        self.detected.remove(&id);
        owner.remove_child(owner.get_node(id)?);
        Some(())
    }
}

fn render_marker(
    owner: &Node,
    player: TRef<Spatial>,
    (id, area): (&GodotString, &Ref<Area>),
) -> Option<()> {
    let node = owner.get_node(id.to_owned())?;

    if let Some(ret) = unsafe { node.assume_safe() }.cast::<TextureRect>() {
        let vec = (unsafe { area.assume_safe() }.global_transform().origin
            - player.global_transform().origin)
            / 5.; // scale
        let rel = Vector2::new(vec.x + 100., vec.y + 100.);

        ret.set_position(rel, false);
        godot_print!("{} || {:?} || {:?}", id, ret, rel);
    }
    Some(())
}
