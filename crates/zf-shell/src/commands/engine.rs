use crate::imports;
use nu_engine::CallExt;
use nu_protocol::{engine::Command, IntoPipelineData, ShellError, Signature, SyntaxShape, Value};
use zf_bridge::{EngineCommand, ZFCommandArgs};

#[derive(Clone)]
pub(crate) struct Engine;

impl Command for Engine {
    fn name(&self) -> &str {
        "engine"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("engine")
    }

    fn usage(&self) -> &str {
        "Operate on engine"
    }

    fn run(
        &self,
        _engine_state: &nu_protocol::engine::EngineState,
        _stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::ast::Call,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, ShellError> {
        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}

#[derive(Clone)]
pub(crate) struct EngineOn;

impl Command for EngineOn {
    fn name(&self) -> &str {
        "engine on"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name())
    }

    fn usage(&self) -> &str {
        "Start engine"
    }

    fn run(
        &self,
        _engine_state: &nu_protocol::engine::EngineState,
        _stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::ast::Call,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, ShellError> {
        let args = ZFCommandArgs::Engine(EngineCommand::On);
        imports::zf_call(args);
        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}

#[derive(Clone)]
pub(crate) struct EngineOff;

impl Command for EngineOff {
    fn name(&self) -> &str {
        "engine off"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name())
    }

    fn usage(&self) -> &str {
        "Turn off engine"
    }

    fn run(
        &self,
        _engine_state: &nu_protocol::engine::EngineState,
        _stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::ast::Call,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, ShellError> {
        let args = ZFCommandArgs::Engine(EngineCommand::Off);
        imports::zf_call(args);
        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}

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
        engine_state: &nu_protocol::engine::EngineState,
        stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::ast::Call,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, ShellError> {
        let t: f64 = call.req(engine_state, stack, 0)?;

        let args = ZFCommandArgs::Engine(EngineCommand::Thruster(t as i8));
        imports::zf_call(args);
        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}
