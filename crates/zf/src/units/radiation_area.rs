use gdnative::{
    api::{object::ConnectFlags, Area},
    prelude::*,
};
use zf_ffi::TaskListenableEvent;

use crate::{common::find_ref, managers::VM, refs::groups, vm::VMSignal};

use super::Player;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct RadiationArea {
    active: bool,
}

#[methods]
impl RadiationArea {
    fn new(_base: &Node) -> Self {
        RadiationArea { active: false }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal(VMSignal::OnListenableEvent.as_str()).done();
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

        let vm_manager = find_ref::<VM, Node>(base).unwrap();

        base.connect(
            VMSignal::OnListenableEvent.as_str(),
            vm_manager,
            VMSignal::OnListenableEvent.as_str(),
            VariantArray::new_shared(),
            ConnectFlags::DEFERRED.into(),
        )
        .expect("failed to connect hit_by_player");

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
    fn on_detected(&mut self, #[base] base: &Node, area: Ref<Area>) -> Option<()> {
        let maybe_player = unsafe { area.assume_safe().get_parent()?.assume_safe() };
        if !maybe_player.is_in_group(groups::PLAYER) {
            return None;
        }
        base.emit_signal(
            VMSignal::OnListenableEvent,
            &[TaskListenableEvent::RadiationAreaEntered.to_variant()],
        );
        self.active = true;
        Some(())
    }

    #[method]
    fn on_lost(&mut self, #[base] base: &Node, area: Ref<Area>) -> Option<()> {
        let maybe_player = unsafe { area.assume_safe().get_parent()?.assume_safe() };
        if !maybe_player.is_in_group(groups::PLAYER) {
            return None;
        }
        base.emit_signal(
            VMSignal::OnListenableEvent,
            &[TaskListenableEvent::RadiationAreaExited.to_variant()],
        );

        self.active = false;
        Some(())
    }
}
