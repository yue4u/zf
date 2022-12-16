use gdnative::{
    api::{object::ConnectFlags, Area, Spatial},
    prelude::*,
};

use crate::{
    common::find_ref, entities::GameEvent, managers::VMManager, refs::groups, vm::VMSignal,
};

#[derive(NativeClass)]
#[inherit(Spatial)]
#[register_with(Self::register_signals)]
pub struct TargetPoint;

const HIT_BY_PLAYER: &'static str = "hit_by_player";

#[methods]
impl TargetPoint {
    fn new(base: &Spatial) -> Self {
        base.add_to_group(groups::TARGET_POINT, false);
        TargetPoint
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

        let as_node = unsafe { base.get_node_as::<Node>(".")? };
        let vm_manager = find_ref::<VMManager, Node>(as_node)?;

        base.connect(
            HIT_BY_PLAYER,
            vm_manager,
            VMSignal::OnGameState.as_str(),
            VariantArray::new_shared(),
            ConnectFlags::DEFERRED.into(),
        )
        .expect("failed to connect hit_by_player");
        None
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal(HIT_BY_PLAYER).done();
    }

    #[method]
    fn on_area_entered(&self, #[base] base: TRef<Spatial>, area: Ref<Spatial>) -> Option<()> {
        let area_parent = unsafe { area.assume_safe().get_parent()?.assume_safe() };
        let is_player = area_parent.is_in_group(groups::PLAYER);
        if !is_player {
            return None;
        }

        base.emit_signal(HIT_BY_PLAYER, &[GameEvent::HitTargetPoint.to_variant()]);
        base.queue_free();

        None
    }
}
