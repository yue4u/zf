use gdnative::{
    api::{RandomNumberGenerator, WorldEnvironment},
    prelude::*,
};

use crate::{
    common::{self, find_ref},
    refs::{
        groups::Group,
        path::{self, scenes},
    },
    units::Player,
};

use super::HomingMissile;

#[derive(NativeClass, Debug)]
#[inherit(Node)]
pub struct Launcher {
    #[property(default = 1000)]
    random_start_time_msec: i32,

    #[property(default = 1000)]
    wait_time_msec: i32,

    timer: Option<Ref<Timer>>,

    emit_obj_path: Option<String>,

    target_player: bool,
}

#[methods]
impl Launcher {
    fn new(base: &Node) -> Self {
        // it looks we need duplicate defaults here 😐
        // https://github.com/godot-rust/godot-rust/blob/29b89b0eb3ab0e053dc9702f9b1ac29dca4ecf22/examples/dodge-the-creeps/src/mob.rs#L36-L41
        Launcher {
            random_start_time_msec: 1000,
            wait_time_msec: 1000,
            timer: None,
            emit_obj_path: None,
            target_player: true,
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Node>) {
        godot_dbg!("{:?}", &self);

        let mut rng = RandomNumberGenerator::new();
        let start_time_msec = rng.randi_range(0, self.random_start_time_msec as i64);
        godot_dbg!("rng says {}", start_time_msec);
        let timer = unsafe { Timer::new().into_shared().assume_safe() };
        base.add_child(timer, false);

        timer.set_one_shot(true);
        timer.start(self.random_start_time_msec as f64 / 1000.);
        timer
            .connect("timeout", base, "start", VariantArray::new_shared(), 0)
            .unwrap();
        self.timer = Some(timer.claim());
    }

    #[method]
    fn start(&self, #[base] base: TRef<Node>) {
        let timer = unsafe { self.timer.unwrap().assume_safe() };
        timer.disconnect("timeout", base, "start");
        timer.set_one_shot(false);
        timer
            .connect("timeout", base, "trigger", VariantArray::new_shared(), 0)
            .unwrap();
        timer.start(self.wait_time_msec as f64 / 1000.);
    }

    #[method]
    fn trigger(&self, #[base] base: TRef<Node>) {
        let obj = common::load_as::<Spatial>(
            self.emit_obj_path
                .as_deref()
                .unwrap_or(scenes::HOMING_MISSILE),
        )
        .unwrap();
        obj.set_global_transform(
            unsafe { base.get_parent().unwrap().assume_safe() }
                .cast::<Spatial>()
                .unwrap()
                .global_transform(),
        );
        let missile = obj.cast_instance::<HomingMissile>().unwrap();
        let player_pos = find_ref::<Player, Spatial>(base)
            .unwrap()
            .global_transform()
            .origin;
        missile
            .map_mut(|m, _| {
                if self.target_player {
                    m.group = Group::ENEMY;
                }
                m.target_pos = Some(player_pos)
            })
            .unwrap();

        unsafe {
            base.get_node(path::base_level::PROJECTILES)
                .unwrap()
                .assume_safe()
        }
        .add_child(missile, false);
    }
}
