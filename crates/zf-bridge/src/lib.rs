pub use bincode::*;

#[derive(Decode, Encode, Debug, PartialEq)]
pub enum CommandBridge {
    // Help,
    Game(GameCommand),
    // Mission(MissionCommand),
    Engine(EngineCommand),
    // Autopilot(AutopilotCommand),
    // Unkonwn(String),
    // Fire(FireCommand),
    // Radar(RadarCommand),
    // UI(UICommand),
    /// up to host impl and could use for test
    Mystery,
}

#[derive(Decode, Encode, Debug, PartialEq)]
pub enum GameCommand {
    Start,
    Menu,
    End,
}

#[derive(Decode, Encode, Debug, PartialEq)]
pub enum EngineCommand {
    On,
    Off,
    Thruster(i8),
}

pub struct Tag;

// multi value wasm compilation does not work yet
// so conbine two i32 to i64 and convert them back
// https://github.com/rust-lang/rust/issues/73755
impl Tag {
    pub fn into(ptr: i32, len: i32) -> i64 {
        (ptr as i64) << 32 | (len as i64)
    }

    pub fn from(tag: i64) -> (i32, i32) {
        let len = tag as i32;
        let ptr = (tag >> 32) as i32;
        (ptr, len)
    }
}
