use crate::vm::Command;

pub trait Execute {
    fn exec(&self) -> ExecuteResult;
}

pub type ExecuteResult = Result<String, ()>;

pub fn exec(cmd: Command) -> String {
    let mut text = None;

    if let Ok(result) = cmd.exec() {
        text = Some(result)
    }

    text.unwrap_or(format!("{:?}", cmd))
}
