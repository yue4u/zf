use gdnative::{api::RichTextLabel, prelude::*};

// use crate::{refs::HasPath, units::Player};

#[derive(NativeClass, Default)]
#[inherit(RichTextLabel)]
pub struct PlayerStatusDisplay;

#[methods]
impl PlayerStatusDisplay {
    fn new(_base: &RichTextLabel) -> Self {
        PlayerStatusDisplay::default()
    }

    #[method]
    fn _ready(&self) -> Option<()> {
        godot_print!("player status ready");
        Some(())
    }

    #[method]
    fn _process(&self, #[base] base: &RichTextLabel, _delta: f64) -> Option<()> {
        self.sync(base)
    }

    fn sync(&self, base: &RichTextLabel) -> Option<()> {
        // unsafe { base.get_node_as_instance::<Player>(Player::path())? }
        //     .map(|p, _| {
        //         base.set_bbcode(p.display());
        //     })
        //     .ok();
        None
    }
}
