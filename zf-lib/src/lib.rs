mod command_palette;
mod env;
mod hello;

use gdnative::prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<hello::Hello>();
    handle.add_class::<env::Env>();
    handle.add_class::<command_palette::CommandPalette>();
}

godot_init!(init);
