use std::f64::consts::PI;

use gdnative::api::*;
use gdnative::prelude::*;

use super::DamageAble;
use super::Weapon;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct HomingMissile {
    #[property]
    pub target_pos: Option<Vector3>,
}

#[methods]
impl HomingMissile {
    const DAMGAE: u32 = 50;

    fn new(_base: &Spatial) -> Self {
        HomingMissile { target_pos: None }
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Spatial>) {
        base.prepare_as_weapon();
    }

    #[method]
    fn _process(&self, #[base] base: &Spatial, delta: f32) {
        if let Some(local_target_pos) = self.target_pos {
            // FIXME: better to always use global transform?
            let mut local_t = base.transform();
            local_t.origin = local_t.origin.move_toward(local_target_pos, delta * 20.);

            base.set_transform(local_t);

            if local_t.origin.distance_to(local_target_pos) > 0.01 {
                base.look_at(local_t.origin.direction_to(local_target_pos), Vector3::UP);
                base.rotate_object_local(Vector3::UP, PI)
            } else {
                base.queue_free()
            }
        } else {
            base.translate(Vector3::new(0.0, 0.0, -delta * 300.0));
        }
    }

    #[method]
    fn on_detected(&self, #[base] base: &Spatial, area: Ref<Area>) {
        if area.try_damage(Self::DAMGAE).is_ok() {
            base.queue_free();
        }
    }

    #[method]
    fn on_timeout_queue_free(&self, #[base] base: &Spatial) {
        base.queue_free()
    }
}
