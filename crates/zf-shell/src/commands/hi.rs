use nu_protocol::{engine::Command, IntoPipelineData, ShellError, Signature, Value};

#[derive(Clone)]
pub(crate) struct Hi;

impl Command for Hi {
    fn name(&self) -> &str {
        "hi"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("hi")
    }

    fn usage(&self) -> &str {
        "hi"
    }

    fn run(
        &self,
        _engine_state: &nu_protocol::engine::EngineState,
        _stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::ast::Call,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, ShellError> {
        Ok(Value::String {
            val: "hi!".into(),
            span: call.head,
        }
        .into_pipeline_data())
    }
}
