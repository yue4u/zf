use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, Value,
};

#[derive(Clone)]
pub(crate) struct Hi;

impl Command for Hi {
    fn name(&self) -> &str {
        "hi"
    }

    fn signature(&self) -> Signature {
        Signature::build("hi")
    }

    fn usage(&self) -> &str {
        "hi"
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        Ok(Value::String {
            val: "hi!".into(),
            span: call.head,
        }
        .into_pipeline_data())
    }
}
