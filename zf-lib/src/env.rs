use gdnative::{api::WorldEnvironment, prelude::*};

#[derive(NativeClass)]
#[inherit(Node)]
pub(crate) struct Env;

#[methods]
impl Env {
    fn new(_owner: &Node) -> Self {
        Env
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("env ready");
    }

    #[export]
    fn _process(&self, owner: &Node, delta: f64) -> Option<()> {
        let env = owner.cast::<WorldEnvironment>()?.environment()?;
        let env = unsafe { env.assume_safe() };
        let mut degrees = env.sky_rotation_degrees();
        degrees.y -= delta as f32;
        env.set_sky_rotation_degrees(degrees);
        Some(())
    }
}
