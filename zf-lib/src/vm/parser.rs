use crate::vm::{Command, InvalidCommand};

pub struct Parser;

impl Parser {
    pub fn parse(input: String) -> Result<Vec<Command>, InvalidCommand> {
        input
            .split('|')
            .map(|cmd| Command::try_from(cmd.trim()))
            .collect::<Result<Vec<Command>, InvalidCommand>>()
    }
}
