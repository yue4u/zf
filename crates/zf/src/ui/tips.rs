use crate::{
    common::{current_scene, SceneName},
    vm::{CommandInput, VMConnecter, VMSignal},
};
use gdnative::{api::RichTextLabel, prelude::*};

#[derive(NativeClass)]
#[inherit(RichTextLabel)]
pub struct Tips;

#[methods]
impl Tips {
    fn new(_base: &RichTextLabel) -> Self {
        Tips
    }

    #[method]
    fn _ready(&self, #[base] base: &RichTextLabel) -> Option<()> {
        // godot_print!("tips ready");
        let as_node = unsafe { base.get_node_as::<Node>(".")? };
        as_node.connect_vm_signal(VMSignal::OnCmdParsed.to_options().bidirectional(false));
        let text = match current_scene(&as_node) {
            SceneName::Sandbox => "",
            SceneName::StartMenu => "Type [b][color=#FFC23C]game start[/color][/b] to continue or [b][color=#FFC23C]help[/color][/b] for help.",
            _ => "",
        };
        base.update_label(text);
        Some(())
    }

    #[method]
    fn on_cmd_parsed(&self, #[base] base: &RichTextLabel, input: CommandInput) {
        base.update_label(&format!("{:?}", input));
    }
}

trait UpdateLabel {
    fn update_label(&self, text: &str);
}

impl UpdateLabel for RichTextLabel {
    fn update_label(&self, text: &str) {
        self.set_visible(!text.is_empty());
        self.set_bbcode(text);
    }
}
