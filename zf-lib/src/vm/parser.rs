pub struct Parser;

#[derive(Debug)]
pub enum MissionCommand {
    Summary,
}

#[derive(Debug)]
pub struct Misson {
    title: String,
    info: String,
}

#[derive(Debug)]
pub enum Command {
    Game(GameCommand),
    Mission(MissionCommand),
    Unkonwn(String),
}

#[derive(Debug)]
pub enum GameCommand {
    Start,
    Stop,
}

#[derive(Debug)]
pub struct InvalidCommand;

impl TryFrom<String> for Command {
    type Error = InvalidCommand;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let args = value.split(' ');
        let ns = args.into_iter().collect::<Vec<&str>>();
        match &ns[..] {
            &["game" | "g", act] => match act {
                "start" => Some(Command::Game(GameCommand::Start)),
                "stop" => Some(Command::Game(GameCommand::Stop)),
                _ => None,
            },
            &["mission" | "m", ..] => match &ns[1..] {
                ["summary"] | [] => Some(Command::Mission(MissionCommand::Summary)),
                _ => None,
            },
            _ => Some(Command::Unkonwn(value)),
        }
        .ok_or(InvalidCommand)
    }
}

impl Parser {
    pub fn parse(input: String) -> Result<Command, InvalidCommand> {
        Command::try_from(input)
    }
}
