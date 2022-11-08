use gdnative::api::*;
use gdnative::prelude::*;

use crate::refs::groups::Group;
use crate::units::Player;
use crate::units::TDummy;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct HomingMissile {
    pub target_pos: Option<Vector3>,
    pub group: Group,
}

#[methods]
impl HomingMissile {
    fn new(_base: &Spatial) -> Self {
        godot_print!("prepare HomingMissile");
        HomingMissile {
            target_pos: None,
            group: Group::ENEMY,
        }
    }

    #[method]
    fn _ready(&self, #[base] base: TRef<Spatial>) {
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

        unsafe {
            base.get_node("Timer")
                .expect("child Timer should exist")
                .assume_safe()
        }
        .connect("timeout", base, "on_timeout", VariantArray::new_shared(), 0)
        .expect("failed to connect area_entered");
    }

    #[method]
    fn _process(&self, #[base] base: &Spatial, delta: f64) -> Option<()> {
        let delta = (delta as f32) * 300.0;

        if let Some(target_pos) = self.target_pos {
            let mut t = base.global_transform();
            t.origin = t.origin.move_toward(target_pos, delta);

            base.look_at(target_pos, Vector3::UP);
            base.set_global_transform(t);
        } else {
            base.translate(Vector3::new(0.0, 0.0, -delta));
        }
        Some(())
    }

    #[method]
    unsafe fn on_detected(&self, #[base] base: &Spatial, area: Ref<Area>) -> Option<()> {
        let spatial = area
            .assume_safe()
            .get_parent()?
            .assume_safe()
            .cast::<Spatial>()?;
        match self.group {
            Group::PLAYER => {
                spatial
                    .cast_instance::<TDummy>()?
                    .map(|d, _| {
                        d.damage();
                    })
                    .unwrap();
            }
            Group::ENEMY => {
                spatial
                    .cast_instance::<Player>()?
                    .map(|p, _| {
                        p.damage();
                    })
                    .unwrap();
            }
        }
        base.queue_free();
        Some(())
    }

    #[method]
    fn on_timeout(&self, #[base] base: &Spatial) {
        godot_dbg!("HomingMissile queue_free");
        base.queue_free()
    }
}
