use gdnative::{api::AnimationPlayer, prelude::*};

#[derive(NativeClass)]
#[inherit(AnimationPlayer)]
pub struct AnimationManager {
    #[property]
    name: Option<String>,
}

#[methods]
impl AnimationManager {
    fn new(_owner: &AnimationPlayer) -> Self {
        AnimationManager { name: None }
    }

    #[export]
    fn _ready(&self, owner: &AnimationPlayer) -> Option<()> {
        owner.play(self.name.as_ref()?, -1.0, 0.05, false);
        Some(())
    }
}
