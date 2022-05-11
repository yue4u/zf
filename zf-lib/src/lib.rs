mod ui;
mod vm;
mod common;
mod entities;

use gdnative::prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<ui::Hello>();
    handle.add_class::<ui::Env>();
    handle.add_class::<ui::CommandPalette>();
    handle.add_class::<ui::CommandHistory>();
    handle.add_class::<ui::CommandResult>();
}

godot_init!(init);
