use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct HomingMissile;

#[methods]
impl HomingMissile {
    fn new(_base: &Spatial) -> Self {
        godot_print!("prepare HomingMissile");
        HomingMissile
    }

    #[method]
    fn _process(&self, #[base] base: &Spatial, delta: f64) -> Option<()> {
        base.translate(Vector3::new(0.0, 0.0, (-delta as f32) * 500.0));
        Some(())
    }
}
