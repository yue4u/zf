use gdnative::api::*;
use gdnative::prelude::*;

use super::DamageAble;
use super::Weapon;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct Beam {
    #[property]
    pub target_pos: Option<Vector3>,
    pub direction: Option<Vector3>,
}

#[methods]
impl Beam {
    const DAMAGE: u32 = 50;

    fn new(_base: &Spatial) -> Self {
        Beam {
            target_pos: None,
            direction: None,
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Spatial>) {
        if let Some(target_pos) = self.target_pos.take() {
            self.direction = Some(base.global_transform().origin.direction_to(target_pos))
        }
        base.prepare_as_weapon();
    }

    #[method]
    fn _process(&self, #[base] base: &Spatial, delta: f64) {
        let delta = (delta as f32) * 300.0;

        base.global_translate(
            self.direction
                .map(|d| d * delta)
                .unwrap_or_else(|| Vector3::new(0.0, 0.0, -delta)),
        );
    }

    #[method]
    unsafe fn on_detected(&self, #[base] base: &Spatial, area: Ref<Area>) {
        if area.try_damage(Self::DAMAGE).is_ok() {
            base.queue_free()
        }
    }

    #[method]
    fn on_timeout_queue_free(&self, #[base] base: &Spatial) {
        base.queue_free()
    }
}
