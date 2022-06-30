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
        // let mut transform = owner.global_transform();
        // transform
        //     .basis
        //     .set_b(transform.basis.b() * (1.0 + delta as f32));
        // owner.set_global_transform(transform);

        Some(())
    }
}
