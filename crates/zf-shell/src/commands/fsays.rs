use ferris_says::say;
use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Value,
};

#[derive(Clone)]
pub(crate) struct Fsays;

impl Command for Fsays {
    fn name(&self) -> &str {
        "fsays"
    }

    fn signature(&self) -> Signature {
        Signature::build("msg").optional("message", SyntaxShape::String, "message")
    }

    fn usage(&self) -> &str {
        "fsays hi"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let msg = call
            .req::<String>(engine_state, stack, 0)
            .or_else(|_| input.into_value(call.head).as_string())?;

        let mut vec = Vec::new();
        say(&msg, 40, &mut vec)?;

        let val = String::from_utf8(vec).unwrap();

        Ok(Value::String {
            val,
            span: call.span(),
        }
        .into_pipeline_data())
    }
}
