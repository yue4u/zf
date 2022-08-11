mod common;
mod entities;
mod managers;
mod path;
mod ui;
mod units;
mod vm;
mod weapons;
use gdnative::prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<ui::Hello>();
    handle.add_class::<ui::Env>();
    handle.add_class::<ui::CommandPalette>();
    handle.add_class::<ui::CommandHistory>();
    handle.add_class::<ui::CommandResultDisplay>();
    handle.add_class::<ui::PlayerStatusDisplay>();
    handle.add_class::<ui::Radar>();
    handle.add_class::<ui::PerfLabel>();
    handle.add_class::<ui::Tips>();
    handle.add_class::<units::GangutSpaceHub>();
    handle.add_class::<units::Player>();
    handle.add_class::<weapons::HomingMissile>();
    handle.add_class::<managers::VMManager>();
    handle.add_class::<managers::AnimationManager>();
    handle.add_class::<managers::SysManager>();
}

godot_init!(init);
