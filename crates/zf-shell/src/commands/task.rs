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
        Signature::build(self.name()).required(
            "cmd", //
            SyntaxShape::String,
            "cmd to run",
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
        let args = CommandArgs::Task(TaskCommand::Run { cmd });
        let val = zf_ffi::cmd_legacy(args);
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
        let val = zf_ffi::cmd_legacy(args);
        Ok(Value::String {
            val,
            span: call.head,
        }
        .into_pipeline_data())
    }
}

#[derive(Clone)]
pub(crate) struct TaskOn;

impl Command for TaskOn {
    fn name(&self) -> &str {
        "task on"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name())
            .required(
                "event", //
                SyntaxShape::String,
                "event to listen",
            )
            .required(
                "cmd", //
                SyntaxShape::String,
                "cmd to run",
            )
    }

    fn usage(&self) -> &str {
        "Run a cmd in when a event happended"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let event = &call.req::<String>(engine_state, stack, 0)?;
        let cmd: String = call.req(engine_state, stack, 1)?;

        let args = CommandArgs::Task(TaskCommand::On {
            event: event.as_str().try_into().map_err(|err| {
                ShellError::IncompatibleParametersSingle(err, call.positional_nth(0).unwrap().span)
            })?,
            cmd,
        });
        let val = zf_ffi::cmd_legacy(args);
        Ok(Value::String {
            val,
            span: call.head,
        }
        .into_pipeline_data())
    }
}
