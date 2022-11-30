use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Value,
};
use zf_ffi::{CommandArgs, EngineCommand};

use crate::cmd;

cmd::empty!(
    Engine,
    name: "engine",
    usage: "engine"
);

cmd::proxy!(
    EngineOn,
    name: "engine on",
    usage: "Turn on engine",
    arg: CommandArgs::Engine(EngineCommand::On)
);

cmd::proxy!(
    EngineOff,
    name: "engine off",
    usage: "Turn off engine",
    arg: CommandArgs::Engine(EngineCommand::Off)
);

#[derive(Clone)]
pub(crate) struct EngineThruster;

impl Command for EngineThruster {
    fn name(&self) -> &str {
        "engine thruster"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name()).required(
            "number", //
            SyntaxShape::Int,
            "engine thruster number to set",
        )
    }

    fn usage(&self) -> &str {
        "Set engine thruster"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        thruster(engine_state, stack, call, _input)
    }
}

#[derive(Clone)]
pub(crate) struct EngineThrusterShort;

impl Command for EngineThrusterShort {
    fn name(&self) -> &str {
        "engine t"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name()).required(
            "number", //
            SyntaxShape::Int,
            "engine thruster number to set",
        )
    }

    fn usage(&self) -> &str {
        "Set engine thruster"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        thruster(engine_state, stack, call, _input)
    }
}

fn thruster(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    _input: PipelineData,
) -> Result<PipelineData, ShellError> {
    let t: f64 = call.req(engine_state, stack, 0)?;

    let args = CommandArgs::Engine(EngineCommand::Thruster(t as i8));
    zf_ffi::cmd(args);
    Ok(Value::Nothing { span: call.head }.into_pipeline_data())
}
