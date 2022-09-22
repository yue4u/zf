use gdnative::{api::WorldEnvironment, prelude::*};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Env;

#[methods]
impl Env {
    fn new(_base: &Node) -> Self {
        Env
    }

    #[method]
    fn _ready(&self) {
        godot_print!("env ready");
    }

    #[method]
    fn _process(&self, #[base] base: &Node, delta: f64) -> Option<()> {
        let env = base.cast::<WorldEnvironment>()?.environment()?;
        let env = unsafe { env.assume_safe() };
        let mut degrees = env.sky_rotation_degrees();
        degrees.y -= delta as f32;
        env.set_sky_rotation_degrees(degrees);
        Some(())
    }
}
