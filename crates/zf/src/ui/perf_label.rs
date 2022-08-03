use gdnative::{
    api::{Engine, Performance},
    prelude::*,
};

#[derive(NativeClass)]
#[inherit(Label)]
pub struct PerfLabel;

#[methods]
impl PerfLabel {
    fn new(_owner: &Label) -> Self {
        PerfLabel
    }

    #[export]
    fn _ready(&self, _owner: &Label) {
        godot_print!("perf label ready");
    }

    #[export]
    fn _process(&self, owner: &Label, _delta: f64) -> Option<()> {
        let engine = Engine::godot_singleton();
        let fps = engine.get_frames_per_second();
        let mem =
            Performance::godot_singleton().get_monitor(Performance::MEMORY_STATIC) / 1024. / 1024.;
        owner.set_text(format!(
            r#"fps: {fps}
mem: {mem:.1}m
"#
        ));
        Some(())
    }
}
