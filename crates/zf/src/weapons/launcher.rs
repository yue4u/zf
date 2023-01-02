use gdnative::{
    api::{Area, RandomNumberGenerator},
    prelude::*,
};

use crate::{
    common::SceneLoader,
    refs::{
        self,
        groups::Layer,
        path::{base_level, scenes},
    },
    units::Player,
};

impl Launcher {
    pub fn load_with_weapon(base: TRef<Spatial>, weapon_path: impl ToString) {
        let node = SceneLoader::load_and_instance_as::<Node>(refs::path::scenes::LAUNCHER)
            .expect("load failed");
        let launcher = node
            .cast_instance::<Launcher>()
            .expect("cast_instance failed")
            .into_shared();

        unsafe { launcher.assume_safe() }
            .map_mut(|l, _| {
                l.weapon_path = Some(weapon_path.to_string());
            })
            .expect("update weapon_path failed");

        base.add_child(launcher.base(), false);
    }
}

#[derive(NativeClass, Debug)]
#[inherit(Node)]
pub struct Launcher {
    #[property(default = 1000)]
    random_start_time_msec: i32,

    #[property(default = 300)]
    wait_time_msec: i32,

    timer: Option<Ref<Timer>>,

    pub weapon_path: Option<String>,

    layer: Layer,
}

#[methods]
impl Launcher {
    fn new(_base: &Node) -> Self {
        // it looks we need to duplicate the defaults here 😐
        // https://github.com/godot-rust/godot-rust/blob/29b89b0eb3ab0e053dc9702f9b1ac29dca4ecf22/examples/dodge-the-creeps/src/mob.rs#L36-L41
        Launcher {
            random_start_time_msec: 1000,
            wait_time_msec: 300,
            timer: None,
            weapon_path: None,
            layer: Layer::ENEMY_FIRE,
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Node>) {
        let rng = RandomNumberGenerator::new();
        rng.randomize();
        let start_time_msec = rng.randi_range(0, self.random_start_time_msec as i64);
        // tracing::debug!("{:?}","rng says {}", start_time_msec);
        let timer = unsafe { Timer::new().into_shared().assume_safe() };
        base.add_child(timer, false);

        timer.set_one_shot(true);
        timer.start(start_time_msec as f64 / 1000.);
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
        let weapon = SceneLoader::load_and_instance_as::<Spatial>(
            self.weapon_path
                .as_deref()
                .unwrap_or(scenes::HOMING_MISSILE),
        )
        .unwrap();

        let area = unsafe { weapon.get_node_as::<Area>("Area") }.unwrap();
        self.layer.prepare_collision_for(area);

        let player_pos = unsafe {
            base.get_node(Player::path_from(base.as_ref()))
                .unwrap()
                .assume_safe()
        }
        .cast::<Spatial>()
        .unwrap()
        .global_transform()
        .origin;

        let parent = unsafe { base.get_parent().unwrap().assume_safe() }
            .cast::<Spatial>()
            .unwrap();

        weapon.set_global_transform(parent.global_transform());
        weapon.set_as_toplevel(true);
        weapon.set("target_pos", Some(player_pos));

        unsafe {
            base.get_node(base_level::PROJECTILES)
                .unwrap()
                .assume_safe()
        }
        .add_child(weapon, false);
    }
}
