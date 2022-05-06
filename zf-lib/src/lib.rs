mod env;
mod hello;

use gdnative::prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<hello::Hello>();
    handle.add_class::<env::Env>();
}

godot_init!(init);
