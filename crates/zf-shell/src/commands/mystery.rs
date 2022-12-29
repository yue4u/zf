use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, Value,
};
use zf_ffi::CommandArgs;

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
        "this is a mysterious command"
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let args = CommandArgs::Mystery;
        let val = zf_ffi::cmd_legacy(args);
        Ok(Value::String {
            val,
            span: call.head,
        }
        .into_pipeline_data())
    }
}