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
        // tracing::info!("prepare HomingMissile");
        HomingMissile { target_pos: None }
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Spatial>) {
        base.prepare_as_weapon();
    }

    #[method]
    fn _process(&self, #[base] base: &Spatial, delta: f64) {
        let delta = (delta as f32) * 300.0;

        if let Some(target_pos) = self.target_pos {
            let mut t = base.global_transform();
            t.origin = t.origin.move_toward(target_pos, delta);

            if t.origin.distance_to(target_pos) > 0.1 {
                base.look_at(target_pos, Vector3::UP);
            }
            base.set_global_transform(t);
        } else {
            base.translate(Vector3::new(0.0, 0.0, -delta));
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
        // tracing::debug!("{:?}","HomingMissile queue_free");
        base.queue_free()
    }
}
