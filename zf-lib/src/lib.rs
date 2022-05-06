mod env;
mod hello;
mod command;

use gdnative::prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<hello::Hello>();
    handle.add_class::<env::Env>();
    handle.add_class::<command::CommandPalette>();
    handle.add_class::<command::CommandHistory>();
}

godot_init!(init);
