use gdnative::api::*;
use gdnative::prelude::*;

use crate::units::TDummy;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct HomingMissile {
    pub target_pos: Option<Vector3>,
}

#[methods]
impl HomingMissile {
    fn new(_base: &Spatial) -> Self {
        godot_print!("prepare HomingMissile");
        HomingMissile { target_pos: None }
    }

    #[method]
    fn _ready(&self, #[base] base: TRef<Spatial>) {
        base.set_scale(Vector3::new(0.05, 0.05, 0.05));

        unsafe {
            base.get_node("Area")
                .expect("child Area should exist")
                .assume_safe()
        }
        .connect(
            "area_entered",
            base,
            "on_detected",
            VariantArray::new_shared(),
            0,
        )
        .expect("failed to connect area_entered");
    }

    #[method]
    fn _process(&self, #[base] base: &Spatial, delta: f64) -> Option<()> {
        let delta = (delta as f32) * 500.0;

        if let Some(target_pos) = self.target_pos {
            let mut t = base.global_transform();
            t.origin = t.origin.move_toward(target_pos, delta);
            base.set_global_transform(t);
        } else {
            base.translate(Vector3::new(0.0, 0.0, -delta));
        }
        Some(())
    }

    #[method]
    unsafe fn on_detected(&self, #[base] base: &Spatial, area: Ref<Area>) -> Option<()> {
        area.assume_safe()
            .get_parent()?
            .assume_safe()
            .cast::<Spatial>()?
            .cast_instance::<TDummy>()?
            .map(|d, _| {
                d.damage();
            })
            .unwrap();
        base.queue_free();
        Some(())
    }
}
