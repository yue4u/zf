use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Value,
};
use zf_ffi::{CommandArgs, LevelCommand};

use crate::cmd;

cmd::empty!(
    Level,
    name: "level",
    usage: "level"
);

cmd::proxy!(
    LevelRestart,
    name: "level restart",
    usage: "Restart current level",
    arg: CommandArgs::Level(LevelCommand::Restart)
);

cmd::proxy!(
    LevelNext,
    name: "level next",
    usage: "Start next level",
    arg: CommandArgs::Level(LevelCommand::Next)
);

cmd::proxy!(
    LevelList,
    name: "level ls",
    usage: "List all levels",
    arg: CommandArgs::Level(LevelCommand::List)
);

#[derive(Clone)]
pub(crate) struct LevelStart;

impl Command for LevelStart {
    fn name(&self) -> &str {
        "level start"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).named("name", SyntaxShape::String, "level name", Some('n'))
    }

    fn usage(&self) -> &str {
        "start a level by name"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let name = if input.is_nothing() {
            call.get_flag::<String>(engine_state, stack, "name")?
                .ok_or_else(|| {
                    ShellError::MissingParameter("expected level name".to_owned(), call.head)
                })?
        } else {
            input.into_value(call.head).as_string()?
        };

        let args = CommandArgs::Level(LevelCommand::Start(name));
        zf_ffi::cmd(args);
        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}
