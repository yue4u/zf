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
    Mission(MissionCommand),
    Invalid,
}

impl Parser {
    pub fn parse(input: String) -> Command {
        Command::Invalid
    }
}
