use crate::vm::{Command, CommandRun, CommandRunState, InvalidCommand};

pub struct Parser;

impl Parser {
    pub fn parse(input: String) -> Result<CommandRun, InvalidCommand> {
        input
            .split('|')
            .map(|cmd| Command::try_from(cmd.trim()))
            .collect::<Result<Vec<Command>, InvalidCommand>>()
            .map(|cmds| CommandRun {
                cmds,
                state: CommandRunState::Running,
            })
    }
}
