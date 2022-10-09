use crate::imports;
use nu_protocol::{engine::Command, IntoPipelineData, ShellError, Signature, Value};
use zf_bridge::ZFCommandArgs;

#[derive(Clone)]
pub(crate) struct Mystery;

impl Command for Mystery {
    fn name(&self) -> &str {
        "mystery"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name())
    }

    fn usage(&self) -> &str {
        "this is mysterious command"
    }

    fn run(
        &self,
        _engine_state: &nu_protocol::engine::EngineState,
        _stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::ast::Call,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, ShellError> {
        let args = ZFCommandArgs::Mystery;
        let val = imports::zf_call(args);
        Ok(Value::String {
            val,
            span: call.head,
        }
        .into_pipeline_data())
    }
}
