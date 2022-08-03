use gdnative::{api::RichTextLabel, prelude::*};

use crate::{common::HasPath, units::Player};

#[derive(NativeClass, Default)]
#[inherit(RichTextLabel)]
pub struct PlayerStatusDisplay;

#[methods]
impl PlayerStatusDisplay {
    fn new(_owner: &RichTextLabel) -> Self {
        PlayerStatusDisplay::default()
    }

    #[export]
    fn _ready(&self, _owner: TRef<RichTextLabel>) -> Option<()> {
        godot_print!("player status ready");
        Some(())
    }

    #[export]
    fn _process(&mut self, owner: &RichTextLabel, _delta: f64) -> Option<()> {
        self.sync(owner)
    }

    fn sync(&mut self, owner: &RichTextLabel) -> Option<()> {
        unsafe { owner.get_node_as_instance::<Player>(Player::path())? }
            .map(|p, _| {
                owner.set_bbcode(p.display());
            })
            .ok()
    }
}
