use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Value,
};
use zf_ffi::{AudioCommand, CommandArgs};

#[derive(Clone)]
pub(crate) struct AudioVolume;

impl Command for AudioVolume {
    fn name(&self) -> &str {
        "audio volume"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name()).required(
            "number",
            SyntaxShape::Number,
            "audio volume in range -80..24",
        )
    }

    fn usage(&self) -> &str {
        "Set audio volume"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let t: f64 = call.req(engine_state, stack, 0)?;
        if t < -80. || t > 24. {
            return Err(ShellError::IncompatibleParametersSingle(
                "audio volume must be in range -80..24".to_owned(),
                call.positional_nth(0).unwrap().span,
            ));
        }

        let args = CommandArgs::Audio(AudioCommand::Volume(t));
        zf_ffi::cmd_legacy(args);
        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}
