use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct HomingMissile;

#[methods]
impl HomingMissile {
    fn new(_owner: &Spatial) -> Self {
        godot_print!("prepare HomingMissile");
        HomingMissile
    }

    #[export]
    fn _process(&self, owner: &Spatial, delta: f64) -> Option<()> {
        owner.translate(Vector3::new(0.0, 0.0, (-delta as f32) * 500.0));
        Some(())
    }
}
