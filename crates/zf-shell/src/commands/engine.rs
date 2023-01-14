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
    zf_ffi::cmd_legacy(args);
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
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let mut x = None;
        let mut y = None;
        let mut z = None;

        let input_val = input.into_value(call.head);
        if let Ok((cols, vals)) = input_val.as_record() {
            // TODO: skip iter if both found or use a hashmap
            for (col, val) in cols.iter().zip(vals.iter()) {
                match col.as_str() {
                    "pos" => {
                        let list = val.as_list()?;
                        x = Some(list[0].as_f64()? as f32);
                        y = Some(list[1].as_f64()? as f32);
                        z = Some(list[2].as_f64()? as f32);
                        break;
                    }
                    _ => {}
                }
            }
        }
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

        x = x.or(pos[0]);
        y = y.or(pos[1]);
        z = z.or(pos[2]);

        let args = CommandArgs::Engine(EngineCommand::Rel { x, y, z });
        zf_ffi::cmd_legacy(args);
        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}

#[derive(Clone)]
pub(crate) struct EngineRotate;

impl Command for EngineRotate {
    fn name(&self) -> &str {
        "engine rotate"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name()).required(
            "number", //
            SyntaxShape::Int,
            "engine rotate in turn",
        )
    }

    fn usage(&self) -> &str {
        "Set engine rotate"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let t: f64 = call.req(engine_state, stack, 0)?;

        let args = CommandArgs::Engine(EngineCommand::Rotate(t as f32));
        zf_ffi::cmd_legacy(args);
        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}
