use gdnative::{api::Area, prelude::*};

use crate::refs::groups;

use super::Player;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct RadiationArea {
    active: bool,
}

#[methods]
impl RadiationArea {
    fn new(_base: &Node) -> Self {
        RadiationArea { active: false }
    }

    #[method]
    fn _ready(&self, #[base] base: TRef<Node>) {
        let area = unsafe { base.get_node_as::<Area>("./Area").expect("area exist") };

        area.connect(
            "area_entered",
            base,
            "on_detected",
            VariantArray::new_shared(),
            0,
        )
        .expect("failed to connect area_entered");

        area.connect(
            "area_exited",
            base,
            "on_lost",
            VariantArray::new_shared(),
            0,
        )
        .expect("failed to connect area_exited");

        let timer = unsafe { Timer::new().into_shared().assume_safe() };
        base.add_child(timer, false);
    }

    #[method]
    fn _process(&self, #[base] base: &Node, _delta: f64) -> Option<()> {
        if !self.active {
            return None;
        }
        let player = unsafe { base.get_node_as_instance::<Player>(Player::path_from(base))? };
        player.map_mut(|player, _| player.damage(30)).ok()
    }

    #[method]
    fn on_detected(&mut self, #[base] _base: &Node, area: Ref<Area>) -> Option<()> {
        tracing::debug!("on_detected");
        let maybe_player = unsafe { area.assume_safe().get_parent()?.assume_safe() };
        if !maybe_player.is_in_group(groups::PLAYER) {
            return None;
        }
        self.active = true;
        Some(())
    }

    #[method]
    fn on_lost(&mut self, #[base] _base: &Node, area: Ref<Area>) -> Option<()> {
        tracing::debug!("on_lost");
        let maybe_player = unsafe { area.assume_safe().get_parent()?.assume_safe() };
        if !maybe_player.is_in_group(groups::PLAYER) {
            return None;
        }

        self.active = false;
        Some(())
    }
}
