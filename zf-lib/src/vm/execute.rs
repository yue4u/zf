use crate::vm::Parser;

pub trait Execute {
    fn exec(&self) -> ExecuteResult;
}

pub type ExecuteResult = Result<String, ()>;

pub fn exec(text: String) -> String {
    let cmd = Parser::parse(text);
    let mut text = None;
    if let Ok(cmd) = &cmd {
        if let Ok(result) = cmd.exec() {
            text = Some(result)
        }
    }

    text.unwrap_or(format!("{:?}", cmd))
}
