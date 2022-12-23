use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Value,
};
use zf_ffi::{CommandArgs, CommandResults, LevelCommand};

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

#[derive(Clone)]
pub(crate) struct LevelList;

impl Command for LevelList {
    fn name(&self) -> &str {
        "level ls"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
    }

    fn usage(&self) -> &str {
        "List all levels"
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let args = CommandArgs::Level(LevelCommand::List);
        let val = match zf_ffi::cmd(args) {
            CommandResults::Levels(levels) => Value::List {
                vals: levels
                    .into_iter()
                    .map(|val| Value::String {
                        val,
                        span: call.head,
                    })
                    .collect(),
                span: call.head,
            },
            _ => Value::Nothing { span: call.head },
        };
        Ok(val.into_pipeline_data())
    }
}

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
        zf_ffi::cmd_legacy(args);
        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}
