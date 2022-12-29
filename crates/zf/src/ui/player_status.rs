use gdnative::{api::RichTextLabel, prelude::*};

use crate::units::Player;

// use crate::{refs::HasPath, units::Player};

#[derive(NativeClass, Default)]
#[inherit(RichTextLabel)]
pub struct PlayerStatus;

#[methods]
impl PlayerStatus {
    fn new(_base: &RichTextLabel) -> Self {
        PlayerStatus::default()
    }

    #[method]
    fn _ready(&self) -> Option<()> {
        tracing::info!("player status ready");
        Some(())
    }

    #[method]
    fn _process(&self, #[base] base: &RichTextLabel, _delta: f64) -> Option<()> {
        self.sync(base)
    }

    fn sync(&self, base: &RichTextLabel) -> Option<()> {
        unsafe { base.get_node_as_instance::<Player>(Player::path_from(base))? }
            .map(|p, _| {
                base.set_bbcode(p.display());
            })
            .ok();
        None
    }
}
