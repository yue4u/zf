mod common;
mod entities;
mod ui;
mod units;
mod vm;

use gdnative::prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<ui::Hello>();
    handle.add_class::<ui::Env>();
    handle.add_class::<ui::CommandPalette>();
    handle.add_class::<ui::CommandHistory>();
    handle.add_class::<ui::CommandResult>();
    handle.add_class::<units::GangutSpaceHub>();
    handle.add_class::<units::Player>();
}

godot_init!(init);
