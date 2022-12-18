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

#[derive(Clone)]
pub(crate) struct EngineRel;

impl Command for EngineRel {
    fn name(&self) -> &str {
        "engine rel"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name())
            .named(
                "x",
                SyntaxShape::Number,
                "set relative x from orbit",
                Some('x'),
            )
            .named(
                "y",
                SyntaxShape::Number,
                "set relative y from orbit",
                Some('y'),
            )
            .named(
                "z",
                SyntaxShape::Number,
                "set relative z from orbit",
                Some('z'),
            )
            .switch(
                "reset",
                "reset any unprovided relative pos from orbit",
                Some('r'),
            )
    }

    fn usage(&self) -> &str {
        "Set relative pos from orbit"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let reset = call.has_flag("reset");

        let pos = ["x", "y", "z"]
            .iter()
            .map(|name| {
                call.get_flag::<Value>(engine_state, stack, name)?
                    .map(|v| v.as_float().map(|v| v as f32))
                    .or_else(|| if reset { Some(Ok(0.)) } else { None })
                    .transpose()
            })
            .collect::<Result<Vec<Option<f32>>, ShellError>>()?;

        let args = CommandArgs::Engine(EngineCommand::Rel {
            x: pos[0],
            y: pos[1],
            z: pos[2],
        });
        zf_ffi::cmd(args);
        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}
