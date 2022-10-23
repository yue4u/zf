use crate::{common::find_ref, managers::VMManager, vm::VMSignal};
use gdnative::{
    api::{object::ConnectFlags, LineEdit},
    prelude::*,
};

#[derive(NativeClass)]
#[inherit(LineEdit)]
pub struct CommandPalette;

#[methods]
impl CommandPalette {
    fn new(_base: &LineEdit) -> Self {
        Self
    }

    #[method]
    fn _ready(&self, #[base] base: TRef<LineEdit>) -> Option<()> {
        base.set_cursor_position(base.text().len() as i64);
        base.connect(
            "text_entered",
            base,
            "on_text_entered",
            VariantArray::new_shared(),
            0,
        )
        .expect("failed to connect line edit");

        let as_node = unsafe { base.get_node_as::<Node>(".")? };
        let vm_manager = find_ref::<VMManager, Node>(as_node)?;
        base
            .connect(
                "text_entered",
                vm_manager,
                VMSignal::OnCmdEntered,
                VariantArray::new_shared(),
                ConnectFlags::DEFERRED.into(),
            )
            .expect(&format!("failed to connect vm {}", ""));

        base.grab_focus();

        Some(())
    }

    #[method]
    fn on_text_entered(&self, #[base] base: &LineEdit, _text: String) -> Option<()> {
        base.clear();
        Some(())
    }
}
