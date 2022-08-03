use gdnative::prelude::*;

pub trait Executor {
    fn exec(&self, owner: &Node);
}
