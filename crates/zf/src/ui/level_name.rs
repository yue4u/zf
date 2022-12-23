use gdnative::prelude::*;

use crate::common::current_level;

#[derive(NativeClass)]
#[inherit(Label)]
pub struct LevelName;

#[methods]
impl LevelName {
    fn new(_base: &Label) -> Self {
        LevelName
    }

    #[method]
    fn _ready(&self, #[base] base: TRef<Label>) {
        let level = current_level(unsafe { base.get_node(".").unwrap().assume_safe() }.as_ref());
        base.set_text(format!("Level: {}", level));
    }
}
