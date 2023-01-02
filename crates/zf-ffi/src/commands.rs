pub use bincode::*;

#[cfg(feature = "godot")]
use gdnative::prelude::{FromVariant, ToVariant};

#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub enum CommandArgs {
    // Help,
    Game(GameCommand),
    Level(LevelCommand),
    Tutorial,
    Hint,
    Mission(MissionCommand),
    Engine(EngineCommand),
    Shield(ShieldCommand),
    Task(TaskCommand),
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
    End,
}

#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub enum LevelCommand {
    Start(String),
    Restart,
    Next,
    List,
}

#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub enum TaskCommand {
    Run { cmd: String },
    Stop(String),
    Status,
}

#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub enum EngineCommand {
    On,
    Off,
    Thruster(i8),
    Rel {
        x: Option<f32>,
        y: Option<f32>,
        z: Option<f32>,
    },
}
#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub enum ShieldCommand {
    Show,
    On,
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
    Targets,
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
