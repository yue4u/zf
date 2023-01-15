use crate::refs::groups::LEVEL_INDICATOR;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct LevelIndicator {
    #[property]
    pub max_enemy: u32,
}

#[methods]
impl LevelIndicator {
    fn new(_base: &Node) -> Self {
        LevelIndicator { max_enemy: 0 }
    }

    #[method]
    fn _ready(&self, #[base] base: TRef<Node>) {
        base.add_to_group(LEVEL_INDICATOR, true);
    }
}
