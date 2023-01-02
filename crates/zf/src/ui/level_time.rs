use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Label)]
pub struct LevelTime {
    elapsed: f64,
}

#[methods]
impl LevelTime {
    fn new(_base: &Label) -> Self {
        LevelTime { elapsed: 0. }
    }

    #[method]
    fn _process(&mut self, #[base] base: TRef<Label>, delta: f64) {
        self.elapsed += delta * 1000. * 1000.;
        let duration = chrono::Duration::microseconds(self.elapsed as i64);
        let minutes = duration.num_minutes() % 60;
        let seconds = duration.num_seconds() % 60;
        let millis = duration.num_milliseconds() % 60;

        let text = format!("{:02}:{:02}:{:02}", minutes, seconds, millis);
        base.set_text(text);
    }
}
