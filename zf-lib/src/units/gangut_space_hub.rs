use gdnative::{api::PathFollow, prelude::*};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GangutSpaceHub;

#[methods]
impl GangutSpaceHub {
    fn new(_owner: &Node) -> Self {
        GangutSpaceHub
    }

    #[export]
    fn _process(&self, owner: &Node) -> Option<()> {
        // owner.
        Some(())
    }
}
