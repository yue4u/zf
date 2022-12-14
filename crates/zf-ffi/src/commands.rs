pub use bincode::*;

#[cfg(feature = "godot")]
use gdnative::prelude::{FromVariant, ToVariant};

#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub enum CommandArgs {
    // Help,
    Game(GameCommand),
    Mission(MissionCommand),
    Engine(EngineCommand),
    Task(TaskCommand),
    // Autopilot(AutopilotCommand),
    Fire(FireCommand),
    Radar(RadarCommand),
    UI(UICommand),
    Time(TimeCommand),
    Term(TermCommand),
    /// up to host impl and could use for test
    Mystery,
}

#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub enum GameCommand {
    Start,
    Menu,
    Tutorial,
    End,
}

#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub enum TaskCommand {
    Run { cmd: String, every: Option<u64> },
    Stop(String),
    Status,
}

#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub enum EngineCommand {
    On,
    Off,
    Thruster(i8),
}

#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub enum UIAction {
    Hide,
    Show,
}

#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub struct UICommand {
    pub label: String,
    pub action: UIAction,
}

#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub enum MissionCommand {
    Info,
}

#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub struct RadarCommand {
    // TODO: options
}

#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub struct FireCommand {
    pub weapon: String,
    pub target: Option<String>,
    pub pos: Option<(f32, f32, f32)>,
}

#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub struct TimeCommand {
    pub scale: f64,
}

#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub enum TermCommand {
    Opacity(f32),
}
