mod beam;
mod homing_missile;
mod launcher;

pub use beam::Beam;
use gdnative::api::object::ConnectFlags;
use gdnative::api::*;
use gdnative::prelude::*;
pub use homing_missile::HomingMissile;
pub use launcher::Launcher;

trait Weapon {
    fn prepare_as_weapon(&self) {}
}

impl Weapon for TRef<'_, Spatial> {
    fn prepare_as_weapon(&self) {
        unsafe {
            self.get_node("Area")
                .expect("child Area should exist")
                .assume_safe()
        }
        .connect(
            "area_entered",
            *self,
            "on_detected",
            VariantArray::new_shared(),
            ConnectFlags::DEFERRED.into(),
        )
        .expect("failed to connect area_entered");

        let timer = unsafe { Timer::new().into_shared().assume_safe() };
        self.add_child(timer, false);

        timer
            .connect(
                "timeout",
                *self,
                "on_timeout_queue_free",
                VariantArray::new_shared(),
                ConnectFlags::DEFERRED.into(),
            )
            .expect("failed to connect on_timeout_queue_free");

        timer.set_one_shot(true);
        timer.start(10.);
    }
}

trait DamageAble {
    fn try_damage(&self, ammount: u32) -> anyhow::Result<()>;
}

impl DamageAble for Ref<Area> {
    fn try_damage(&self, ammount: u32) -> anyhow::Result<()> {
        let area = unsafe { self.assume_safe() };
        if area.collision_layer() == 0 {
            return Err(anyhow::Error::msg("does not have collision layer"));
        }
        // FIXME: this only works for player's direct child
        // find a way to avoid this
        let spatial = unsafe {
            area.get_parent()
                .expect("expect parent exist")
                .assume_safe()
        }
        .cast::<Spatial>()
        .expect("expect Spatial");

        if !spatial.has_method("damage") {
            return Err(anyhow::Error::msg("does not have collision layer"));
        }

        unsafe {
            spatial.call_deferred("damage", &[ammount.to_variant()]);
        }
        Ok(())
    }
}
