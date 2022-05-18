use crate::entities::Mission;
use crate::vm::{Execute, ExecuteResult};

#[derive(Debug)]
pub enum Command {
    Help,
    Game(GameCommand),
    Mission(MissionCommand),
    Engine(EngineCommand),
    Autopilot(AutopilotCommand),
    Unkonwn(String),
}

const HELP: &'static str = r#"ZF

help, h           Show this help
game, g
    start         Start game
    stop          Stop game
mission, m
    summary, s    Show mission summary
    target, t     Get mission targets info
    position, p   Get mission targets's positions
engine, e
    start         Start engine
    stop          Stop engine
    thruster, t    Set engine thruster at <percentage>
autopilot, a
    target, t     autopilot to <target>
    orbit, o      autopilot use <orbit>
"#;

#[derive(Debug)]
pub struct InvalidCommand;

impl Execute for Command {
    fn exec(&self) -> ExecuteResult {
        Ok(match self {
            Command::Help => HELP.to_owned(),
            Command::Mission(m) => m.exec()?,
            _ => format!("{:?}", self),
        })
    }
}

impl TryFrom<String> for Command {
    type Error = InvalidCommand;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let args = value
            .split(' ')
            .take_while(|&arg| arg != "|")
            .into_iter()
            .collect::<Vec<&str>>();

        match args[..] {
            ["help" | "h"] => Some(Command::Help),
            ["game" | "g", act] => match act {
                "start" => Some(Command::Game(GameCommand::Start)),
                "stop" => Some(Command::Game(GameCommand::Stop)),
                _ => None,
            },
            ["mission" | "m", ..] => match args[1..] {
                ["summary" | "s"] | [] => Some(Command::Mission(MissionCommand::Summary)),
                ["target" | "t"] => Some(Command::Mission(MissionCommand::Tartget)),
                ["position" | "p"] => Some(Command::Mission(MissionCommand::Position)),
                _ => None,
            },
            ["engine" | "e", ..] => match args[1..] {
                ["start"] => Some(Command::Engine(EngineCommand::Start)),
                ["stop"] => Some(Command::Engine(EngineCommand::Stop)),
                ["thruster" | "t", t] => Some(Command::Engine(EngineCommand::Thruster(
                    t.parse::<i8>()
                        .map(|n| {
                            if -100 <= n && n <= 100 {
                                Ok(n)
                            } else {
                                Err(InvalidCommand)
                            }
                        })
                        .or(Err(InvalidCommand))??,
                ))),
                _ => None,
            },
            ["autopilot" | "a", ..] => match args[1..] {
                ["target" | "t", t] => {
                    Some(Command::Autopilot(AutopilotCommand::Tartget(t.to_owned())))
                }
                ["orbit" | "o", o] => {
                    Some(Command::Autopilot(AutopilotCommand::Orbit(o.to_owned())))
                }
                _ => None,
            },
            _ => Some(Command::Unkonwn(value)),
        }
        .ok_or(InvalidCommand)
    }
}

#[derive(Debug)]
pub enum GameCommand {
    Start,
    Stop,
}

#[derive(Debug)]
pub enum MissionCommand {
    Summary,
    Tartget,
    Position,
}

impl Execute for MissionCommand {
    fn exec(&self) -> ExecuteResult {
        let ret = match self {
            MissionCommand::Summary => Mission::dummy().summary(),
            MissionCommand::Tartget => Mission::dummy().targets().join("\n"),
            MissionCommand::Position => format!("{:?}", Mission::dummy().positions()),
        };
        Ok(ret)
    }
}

#[derive(Debug)]
pub enum EngineCommand {
    Start,
    Stop,
    Thruster(i8),
}

#[derive(Debug)]
pub enum AutopilotCommand {
    Tartget(String),
    Orbit(String),
}
