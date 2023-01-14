use gdnative::{
    api::{object::ConnectFlags, Area},
    prelude::*,
};
use zf_ffi::TaskListenableEvent;

use crate::{common::find_ref, managers::VM, refs::groups, vm::VMSignal};

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct EnemySpawner {
    // active: bool,
}

#[methods]
impl EnemySpawner {
    fn new(_base: &Node) -> Self {
        EnemySpawner {}
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
            "on_entered",
            VariantArray::new_shared(),
            0,
        )
        .expect("failed to connect area_entered");

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
    fn on_entered(&mut self, #[base] base: &Node, area: Ref<Area>) -> Option<()> {
        let parent = unsafe { area.assume_safe().get_parent()?.assume_safe() };
        if !parent.is_in_group(groups::PLAYER) {
            return None;
        }

        // if parent
        base.emit_signal(
            VMSignal::OnListenableEvent,
            &[TaskListenableEvent::EnemyAppeared.to_variant()],
        );

        Some(())
    }
}
