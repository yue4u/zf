use gdnative::{
    api::{object::ConnectFlags, Area, Label3D, Spatial},
    prelude::*,
};

use crate::{
    common::find_ref,
    entities::GameEvent,
    managers::VMManager,
    refs::{self, groups},
    vm::VMSignal,
};

#[derive(NativeClass)]
#[inherit(Spatial)]
#[register_with(Self::register_signals)]
pub struct TargetPoint;

const HIT_TARGET_POINT: &'static str = "hit_target_point";

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

        let label = unsafe {
            base.get_node_as::<Label3D>(refs::path::target_point::LABEL_3_D)
                .unwrap()
        };
        let Transform {
            origin: Vector3 { x, y, z },
            ..
        } = base.transform();
        if x.abs() + y.abs() + z.abs() > 1. {
            // HACK: somehow this is -x for rel?
            label.set_text(format!("{}, {y}, {z}", -x));
        } else {
            label.set_visible(false)
        }
        base.connect(
            HIT_TARGET_POINT,
            vm_manager,
            VMSignal::OnGameState.as_str(),
            VariantArray::new_shared(),
            ConnectFlags::DEFERRED.into(),
        )
        .expect("failed to connect hit_by_player");
        None
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal(HIT_TARGET_POINT).done();
    }

    #[method]
    fn on_area_entered(&self, #[base] base: TRef<Spatial>, area: Ref<Spatial>) -> Option<()> {
        let area_parent = unsafe { area.assume_safe().get_parent()?.assume_safe() };
        let is_player = area_parent.is_in_group(groups::PLAYER);
        if !is_player {
            return None;
        }

        base.emit_signal(HIT_TARGET_POINT, &[GameEvent::HitTargetPoint.to_variant()]);
        base.queue_free();

        None
    }
}
