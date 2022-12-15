mod common;
mod entities;
mod managers;
mod refs;
mod ui;
mod units;
mod vm;
mod weapons;
use gdnative::prelude::*;

fn init(handle: InitHandle) {
    common::logging::init();

    handle.add_class::<ui::Hello>();
    handle.add_class::<ui::Env>();
    handle.add_class::<ui::CommandPalette>();
    handle.add_class::<ui::CommandHistory>();
    handle.add_class::<ui::CommandResultDisplay>();
    handle.add_class::<ui::PlayerStatus>();
    handle.add_class::<ui::Radar>();
    handle.add_class::<ui::PerfLabel>();
    handle.add_class::<ui::Tips>();
    handle.add_class::<ui::Terminal>();
    handle.add_class::<ui::ScreenTransition>();
    handle.add_class::<ui::PlayerHealthBar>();
    handle.add_class::<units::GangutSpaceHub>();
    handle.add_class::<units::Player>();
    handle.add_class::<units::TDummy>();
    handle.add_class::<units::TargetPoint>();
    handle.add_class::<units::TargetDummy>();
    handle.add_class::<weapons::Launcher>();
    handle.add_class::<weapons::HomingMissile>();
    handle.add_class::<weapons::Beam>();
    handle.add_class::<managers::VMManager>();
    handle.add_class::<managers::AnimationManager>();
}

godot_init!(init);
