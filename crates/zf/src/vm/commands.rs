use gdnative::derive::{FromVariant, ToVariant};

use crate::vm::CommandInput;

#[derive(FromVariant, ToVariant, Clone, Debug)]
pub enum CommandRunState {
    Done,
    Failed,
    Running,
}

impl Default for CommandRunState {
    fn default() -> Self {
        CommandRunState::Running
    }
}

#[derive(FromVariant, ToVariant, Clone, Default, Debug)]
pub struct CommandRun {
    pub id: u32,
    pub active_id: usize,
    pub cmds: Vec<CommandInput>,
    pub state: CommandRunState,
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
            ["game" | "g", act] => match act {
                "start" => Game(GameCommand::Start),
                "stop" => Game(GameCommand::Stop),
                _ => Invalid,
            },
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
            _ => Unkonwn(value.to_owned()),
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
    Stop,
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
