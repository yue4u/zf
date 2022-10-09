use gdnative::derive::{FromVariant, ToVariant};

use crate::vm::CommandInput;
use zf_bridge::CommandBridge;

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
    Autopilot(AutopilotCommand),
    Unkonwn(String),
    Fire(FireCommand),
    Radar(RadarCommand),
    UI(UICommand),
    Invalid,
}

use Command::*;

#[derive(Debug)]
pub struct InvalidCommand;

impl TryFrom<&str> for Command {
    type Error = InvalidCommand;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let args = value
            .split(' ')
            .take_while(|&arg| arg != "|")
            .into_iter()
            .collect::<Vec<&str>>();

        let cmd = match args[..] {
            ["help" | "h"] => Help,
            ["mission" | "m", ..] => match args[1..] {
                ["summary" | "s"] | [] => Mission(MissionCommand::Summary),
                ["target" | "t"] => Mission(MissionCommand::Tartget),
                ["position" | "p"] => Mission(MissionCommand::Position),
                _ => Invalid,
            },
            ["engine" | "e", ..] => match args[1..] {
                ["on"] => Engine(EngineCommand::On),
                ["off"] => Engine(EngineCommand::Off),
                ["thruster" | "t", t] => Engine(EngineCommand::Thruster(
                    t.parse::<i8>()
                        .map(|n| {
                            if -100 <= n && n <= 100 {
                                Ok(n)
                            } else {
                                Err(InvalidCommand)
                            }
                        })
                        .or(Err(InvalidCommand))??,
                )),
                _ => Invalid,
            },
            ["autopilot" | "a", ..] => match args[1..] {
                ["target" | "t", t] => Autopilot(AutopilotCommand::Tartget(t.to_owned())),
                ["orbit" | "o", o] => Autopilot(AutopilotCommand::Orbit(o.to_owned())),
                _ => Invalid,
            },
            ["radar"] => Radar(RadarCommand {}),
            ["fire" | "f", ..] => match args[1..] {
                ["hm", ..] => Fire(FireCommand {
                    weapon: Weapon::HomingMissile,
                    target: args.get(2).map(|&t| t.to_owned()),
                }),
                _ => Invalid,
            },
            ["ui", action, label] => UI(UICommand {
                label: label.to_owned(),
                action: match action {
                    "s" | "show" => UIAction::Show,
                    "h" | "hide" => UIAction::Hide,
                    _ => return Err(InvalidCommand),
                },
            }),
            _ => Invalid,
        };
        match cmd {
            Invalid => Err(InvalidCommand),
            _ => Ok(cmd),
        }
    }
}

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub enum GameCommand {
    Start,
    Menu,
    End,
}

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub enum MissionCommand {
    Summary,
    Tartget,
    Position,
}

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub enum EngineCommand {
    On,
    Off,
    Thruster(i8),
}

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub enum AutopilotCommand {
    Tartget(String),
    Orbit(String),
}

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub enum Weapon {
    HomingMissile,
}

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub struct FireCommand {
    pub weapon: Weapon,
    pub target: Option<String>,
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
        use zf_bridge as bridge;
        use zf_bridge::CommandBridge as Arg;
        
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
            Arg::Mystery => Command::Invalid,
        }
    }
}
