use std::fmt::Display;

pub use bincode::*;

#[cfg(feature = "godot")]
use gdnative::prelude::{FromVariant, ToVariant};

#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub enum CommandArgs {
    Game(GameCommand),
    Level(LevelCommand),
    Tutorial,
    Hint,
    Engine(EngineCommand),
    Shield(ShieldCommand),
    Task(TaskCommand),
    Fire(FireCommand),
    Radar(RadarCommand),
    UI(UICommand),
    Audio(AudioCommand),
    Time(TimeCommand),
    Term(TermCommand),
    /// up to host impl and currently used for test
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

#[derive(Decode, Encode, Debug, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub enum TaskListenableEvent {
    RadiationAreaEntered,
    RadiationAreaExited,
    EnemyAppeared,
}

impl TaskListenableEvent {
    pub fn all() -> &'static [TaskListenableEvent] {
        &[
            TaskListenableEvent::RadiationAreaEntered,
            TaskListenableEvent::RadiationAreaExited,
            TaskListenableEvent::EnemyAppeared,
        ]
    }
}

impl Display for TaskListenableEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ev = match self {
            TaskListenableEvent::RadiationAreaEntered => "radiation_area_entered",
            TaskListenableEvent::RadiationAreaExited => "radiation_area_exited",
            TaskListenableEvent::EnemyAppeared => "enemy_appeared",
        };
        f.write_str(ev)
    }
}

impl TryFrom<&str> for TaskListenableEvent {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "radiation_area_entered" => Ok(TaskListenableEvent::RadiationAreaEntered),
            "radiation_area_exited" => Ok(TaskListenableEvent::RadiationAreaExited),
            "enemy_appeared" => Ok(TaskListenableEvent::EnemyAppeared),
            _ => Err(format!("unknown event `{}`", value)),
        }
    }
}

#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub enum TaskCommand {
    Run {
        cmd: String,
    },
    Stop(String),
    On {
        event: TaskListenableEvent,
        cmd: String,
    },
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
    // Z only for now
    Rotate(f32),
}
#[derive(Decode, Encode, Debug, PartialEq)]
#[cfg_attr(feature = "godot", derive(Clone, FromVariant, ToVariant))]
pub enum ShieldCommand {
    Show,
    On,
    Off,
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
pub enum AudioCommand {
    Volume(f64),
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
