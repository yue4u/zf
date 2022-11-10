use gdnative::{api::TextureProgress, prelude::*};

use crate::{
    common::LookAtPlauer,
    refs::{groups, path},
    weapons::Launcher,
};

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct TDummy {
    hp: Option<Ref<TextureProgress>>,
    base_ref: Ref<Spatial>,
}

#[methods]
impl TDummy {
    fn new(base: TRef<Spatial>) -> Self {
        godot_print!("prepare TDummy");
        base.add_to_group(groups::ENEMY, false);
        // Launcher::load_with_weapon(base, path::scenes::HOMING_MISSILE);
        Launcher::load_with_weapon(base, path::scenes::BEAM);

        TDummy {
            hp: None,
            base_ref: base.claim(),
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Spatial>) -> Option<()> {
        let hp = unsafe {
            base.get_node("HP/Viewport/Control")
                .expect("expect TextureProgress")
                .assume_safe()
        }
        .cast::<TextureProgress>()
        .expect("expect can cast TextureProgress")
        .claim();
        self.hp = Some(hp);
        Some(())
    }

    #[method]
    fn _process(&self, #[base] base: TRef<Spatial>, _delta: f64) -> Option<()> {
        base.try_look_at_player()
    }

    #[method]
    pub fn damage(&self) {
        let hp = unsafe { self.hp.unwrap().assume_safe() };
        let hp_tmp = hp.value() - 0.5 * hp.max();
        hp.set_value(hp_tmp);
        if hp_tmp < 0. {
            unsafe { self.base_ref.assume_safe() }.queue_free()
        }
    }
}
