use crate::imports;
use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Value,
};
use zf_bridge::{CommandBridge, EngineCommand};

use super::zf_call;

zf_call::empty_command!(
    Engine,
    name: "engine",
    usage: "engine"
);

zf_call::proxy_command!(
    EngineOn,
    name: "engine start",
    usage: "Turn on engine",
    arg: CommandBridge::Engine(EngineCommand::On)
);

zf_call::proxy_command!(
    EngineOff,
    name: "engine off",
    usage: "Turn off engine",
    arg: CommandBridge::Engine(EngineCommand::Off)
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
        let t: f64 = call.req(engine_state, stack, 0)?;

        let args = CommandBridge::Engine(EngineCommand::Thruster(t as i8));
        imports::zf_call(args);
        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}
