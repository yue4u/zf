use crate::vm::{Command, Parser};

pub trait Execute {
    fn exec(&self) -> ExecuteResult;
}

pub type ExecuteResult = Result<String, ()>;

pub fn _eval(text: String) -> String {
    let cmd = Parser::parse(text);

    if let Ok(cmd) = cmd {
        return exec(cmd);
    }

    format!("{:?}", cmd)
}

pub fn exec(cmd: Command) -> String {
    let mut text = None;

    if let Ok(result) = cmd.exec() {
        text = Some(result)
    }

    text.unwrap_or(format!("{:?}", cmd))
}
