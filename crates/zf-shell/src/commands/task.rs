use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Value,
};
use zf_ffi::{CommandArgs, TaskCommand};

use crate::cmd;

cmd::proxy!(
    Task,
    name: "task",
    usage: "manage background tasks",
    arg: CommandArgs::Task(TaskCommand::Status)
);

#[derive(Clone)]
pub(crate) struct TaskRun;

impl Command for TaskRun {
    fn name(&self) -> &str {
        "task run"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name())
            .required(
                "cmd", //
                SyntaxShape::String,
                "cmd to run",
            )
            .named(
                "every",
                SyntaxShape::Duration,
                "repeat every for given duration",
                Some('e'),
            )
    }

    fn usage(&self) -> &str {
        "Run a cmd in background"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let cmd: String = call.req(engine_state, stack, 0)?;
        let every = match call.get_flag(engine_state, stack, "every")? {
            Some(Value::Duration { val, span }) => {
                if val < 0 {
                    return Err(ShellError::IncompatibleParametersSingle(
                        format!("invaid duration  `{}` for task every", val),
                        span,
                    ));
                }
                Some(val as u64)
            }
            _ => None,
        };
        let args = CommandArgs::Task(TaskCommand::Run { every, cmd });
        let val = zf_ffi::cmd(args);
        Ok(Value::String {
            val,
            span: call.head,
        }
        .into_pipeline_data())
    }
}

#[derive(Clone)]
pub(crate) struct TaskStop;

impl Command for TaskStop {
    fn name(&self) -> &str {
        "task stop"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name()).required(
            "cmd", //
            SyntaxShape::String,
            "cmd to stop",
        )
    }

    fn usage(&self) -> &str {
        "Stop a cmd in background"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let cmd: String = call.req(engine_state, stack, 0)?;
        let args = CommandArgs::Task(TaskCommand::Stop(cmd));
        let val = zf_ffi::cmd(args);
        Ok(Value::String {
            val,
            span: call.head,
        }
        .into_pipeline_data())
    }
}
