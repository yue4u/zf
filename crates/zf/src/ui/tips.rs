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
    fn new(_owner: &RichTextLabel) -> Self {
        Tips
    }

    #[export]
    fn _ready(&self, owner: &RichTextLabel) -> Option<()> {
        godot_print!("tips ready");
        let as_node = unsafe { owner.get_node_as::<Node>(".")? };
        as_node.connect_vm_signal(VMSignal::OnCmdParsed.to_options().bidirectional(false));
        let text = match current_scene(&as_node) {
            SceneName::Sandbox => "",
            SceneName::StartMenu => "Type [b][color=#FFC23C]game start[/color][/b] to continue or [b][color=#FFC23C]help[/color][/b] for help.",
            _ => "",
        };
        owner.update_label(text);
        Some(())
    }

    #[export]
    fn on_cmd_parsed(&self, owner: &RichTextLabel, input: CommandInput) {
        owner.update_label(&format!("{:?}", input));
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
