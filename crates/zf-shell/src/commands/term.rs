use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Value,
};
use zf_ffi::{CommandArgs, TermCommand};

#[derive(Clone)]
pub(crate) struct TermOpacity;
// term opacity
impl Command for TermOpacity {
    fn name(&self) -> &str {
        "term opacity"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name()).required(
            "number", //
            SyntaxShape::Number,
            "opacity number in range of 0.0..1.0",
        )
    }

    fn usage(&self) -> &str {
        "Set term opacity"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let val = call.req::<Value>(engine_state, stack, 0).map_err(|_| {
            ShellError::MissingParameter(format!("please provide a opacity"), call.head)
        })?;

        let opacity = val.as_float()? as f32;
        if opacity < 0. || opacity > 1. {
            return Err(ShellError::IncompatibleParametersSingle(
                format!("opacity must be in range of 0.0..1.0"),
                val.span()?,
            ));
        };

        let args = CommandArgs::Term(TermCommand::Opacity(opacity));
        zf_ffi::cmd_legacy(args);
        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}
