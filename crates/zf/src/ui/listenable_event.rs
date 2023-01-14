use std::collections::VecDeque;

use gdnative::{
    api::{object::ConnectFlags, RichTextLabel},
    prelude::*,
};
use zf_ffi::TaskListenableEvent;

use crate::{common::find_ref, managers::VM, vm::VMSignal};

#[derive(NativeClass)]
#[inherit(RichTextLabel)]
pub struct ListenableEvent {
    content: VecDeque<String>,
}

#[methods]
impl ListenableEvent {
    fn new(_base: &RichTextLabel) -> Self {
        ListenableEvent {
            content: VecDeque::new(),
        }
    }

    #[method]
    fn _ready(&self, #[base] base: TRef<RichTextLabel>) -> Option<()> {
        let as_node = unsafe { base.get_node_as::<Node>(".")? };
        let vm_manager = find_ref::<VM, Node>(as_node)?;

        vm_manager
            .connect(
                VMSignal::OnListenableEvent,
                base,
                VMSignal::OnListenableEvent,
                VariantArray::new_shared(),
                ConnectFlags::DEFERRED.into(),
            )
            .expect("failed to connect vm");
        Some(())
    }

    #[method]
    fn on_listenable_event(
        &mut self,
        #[base] base: TRef<RichTextLabel>,
        event: TaskListenableEvent,
    ) -> Option<()> {
        self.content.push_back(event.to_string());
        if self.content.len() > 5 {
            self.content.pop_front();
        }
        let mut out = "[b][color=#4FFFCA]Events[/color][/b]".to_owned();

        // we don't have join("\n") for VecDeque for at home
        for line in self.content.iter() {
            out.push_str("\n");
            out.push_str(line);
        }

        base.set_bbcode(out);
        Some(())
    }
}
