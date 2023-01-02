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
        let duration = time::Duration::microseconds(self.elapsed as i64);
        let minutes = duration.whole_minutes() % 60;
        let seconds = duration.whole_seconds() % 60;
        let millis = duration.whole_milliseconds() % 60;

        let text = format!("{:02}:{:02}:{:02}", minutes, seconds, millis);
        base.set_text(text);
    }
}
