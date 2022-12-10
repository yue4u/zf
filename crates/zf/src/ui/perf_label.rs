use gdnative::{
    api::{Engine, Performance},
    prelude::*,
};

#[derive(NativeClass)]
#[inherit(Label)]
pub struct PerfLabel;

#[methods]
impl PerfLabel {
    fn new(_base: &Label) -> Self {
        PerfLabel
    }

    #[method]
    fn _ready(&self) {
        tracing::info!("perf label ready");
    }

    #[method]
    fn _process(&self, #[base] base: &Label, _delta: f64) -> Option<()> {
        let engine = Engine::godot_singleton();
        let fps = engine.get_frames_per_second();
        let mem =
            Performance::godot_singleton().get_monitor(Performance::MEMORY_STATIC) / 1024. / 1024.;
        base.set_text(format!(
            r#"fps: {fps}
mem: {mem:.1}m
"#
        ));
        Some(())
    }
}
