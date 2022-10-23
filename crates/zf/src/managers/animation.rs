use gdnative::{api::AnimationPlayer, prelude::*};

#[derive(NativeClass)]
#[inherit(AnimationPlayer)]
pub struct AnimationManager {
    #[property]
    name: Option<String>,
}

#[methods]
impl AnimationManager {
    fn new(_base: &AnimationPlayer) -> Self {
        AnimationManager { name: None }
    }

    #[method]
    fn _ready(&self, #[base] base: &AnimationPlayer) -> Option<()> {
        base.play(self.name.as_ref()?, -1.0, 0.05, false);
        Some(())
    }
}
