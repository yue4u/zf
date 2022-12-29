use gdnative::derive::{FromVariant, ToVariant};

use crate::vm::CommandInput;
use zf_ffi::CommandBridge;

#[derive(FromVariant, ToVariant, Clone, Debug, PartialEq)]
pub enum ProcessState {
    Idle,
    Done,
    Error,
    Running,
}

impl Default for ProcessState {
    fn default() -> Self {
        ProcessState::Idle
    }
}

#[derive(FromVariant, ToVariant, Clone, Default, Debug)]
pub struct Process {
    pub id: u32,
    pub active_id: usize,
    pub cmds: Vec<CommandInput>,
    pub state: ProcessState,
}

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub enum Command {
    Help,
    Game(GameCommand),
    Mission(MissionCommand),
    Engine(EngineCommand),
    // Autopilot(AutopilotCommand),
    Unkonwn(String),
    Fire(FireCommand),
    Radar(RadarCommand),
    UI(UICommand),
    Invalid,
}

use Command::*;

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub enum GameCommand {
    Start,
    Menu,
    End,
}

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub enum MissionCommand {
    Info,
}

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub enum EngineCommand {
    On,
    Off,
    Thruster(i8),
}

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub enum SteeringCommand {
    Tartget(String),
    Orbit(String),
}

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub struct FireCommand {
    pub weapon: String,
    pub target: String,
}

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub struct RadarCommand {
    // TODO: options
}

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub enum UIAction {
    Hide,
    Show,
}

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub struct UICommand {
    pub label: String,
    pub action: UIAction,
}

pub trait IntoCommand {
    fn into_command(self) -> Command;
}

impl IntoCommand for CommandBridge {

    #[rustfmt::skip]
    fn into_command(self) -> Command {
        use zf_ffi as bridge;
        use zf_ffi::CommandBridge as Arg;
        
        match self {
            Arg::Game(bridge::GameCommand::Start) => Command::Game(GameCommand::Start),
            Arg::Game(bridge::GameCommand::End) => Command::Game(GameCommand::End),
            Arg::Game(bridge::GameCommand::Menu) => Command::Game(GameCommand::Menu),
            Arg::Engine(bridge::EngineCommand::On) => Command::Engine(EngineCommand::On),
            Arg::Engine(bridge::EngineCommand::Off) => Command::Engine(EngineCommand::Off),
            Arg::Engine(bridge::EngineCommand::Thruster(t)) => Command::Engine(EngineCommand::Thruster(t)),
            Arg::UI(bridge::UICommand { label, action }) => Command::UI(UICommand{label,action: match action {
                bridge::UIAction::Hide => UIAction::Hide,
                bridge::UIAction::Show => UIAction::Show,
            }}),
            Arg::Mission(bridge::MissionCommand::Info) => Command::Mission(MissionCommand::Info),
            Arg::Radar(bridge::RadarCommand{}) => Command::Radar(RadarCommand{}),
            Arg::Fire(bridge::FireCommand{
                weapon,
                target,
            }) => Command::Fire(FireCommand{
                weapon,
                target,
            }),
            Arg::Mystery => Command::Invalid,
        }
    }
}
