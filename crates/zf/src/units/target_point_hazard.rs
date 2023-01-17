use gdnative::{
    api::{Area, Spatial},
    prelude::*,
};

use crate::{refs::groups, weapons::DamageAble};

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct TargetPointHazard;

#[methods]
impl TargetPointHazard {
    const DAMGAE: u32 = 2000;

    fn new(base: &Spatial) -> Self {
        base.add_to_group(groups::TARGET_POINT_HAZARD, false);
        TargetPointHazard
    }

    #[method]
    fn _ready(&self, #[base] base: TRef<Spatial>) -> Option<()> {
        let area = unsafe { base.get_node_as::<Area>("./Area").expect("area exist") };
        area.connect(
            "area_entered",
            base,
            "on_area_entered",
            VariantArray::new_shared(),
            0,
        )
        .expect("failed to connect area_entered");
        tracing::debug!("TargetPointHazard ready");
        None
    }

    #[method]
    fn on_area_entered(&self, #[base] _base: TRef<Spatial>, area: Ref<Area>) -> Option<()> {
        tracing::debug!("on_area_entered");
        let area_parent = unsafe { area.assume_safe().get_parent()?.assume_safe() };
        let is_player = area_parent.is_in_group(groups::PLAYER);
        tracing::debug!("is_player: {}", is_player);
        if !is_player {
            return None;
        }

        tracing::debug!("try_damage");
        if area.try_damage(Self::DAMGAE).is_ok() {
            // do nothing. no need to queue_free this
            // base.queue_free();
        }

        None
    }
}
