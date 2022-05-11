use crate::vm::{Command, InvalidCommand};

pub struct Parser;

impl Parser {
    pub fn parse(input: String) -> Result<Command, InvalidCommand> {
        Command::try_from(input)
    }
}
