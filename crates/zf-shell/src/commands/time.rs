use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Value,
};

use zf_ffi::{CommandArgs, TimeCommand};

use crate::cmd;

cmd::empty!(
    Time,
    name: "time",
    usage: "control game time scale"
);

#[derive(Clone)]
pub(crate) struct TimeScale;

impl Command for TimeScale {
    fn name(&self) -> &str {
        "time scale"
    }

    fn signature(&self) -> Signature {
        Signature::build("time").required("scale", SyntaxShape::Number, "time scale")
    }

    fn usage(&self) -> &str {
        "time scale 0.5"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let val = call.req::<Value>(engine_state, stack, 0).map_err(|_| {
            ShellError::MissingParameter(format!("please provide a scale"), call.head)
        })?;

        let scale = val.as_float()?;
        if scale < 0. || scale > 5. {
            return Err(ShellError::IncompatibleParametersSingle(
                format!("scale must be in range of 0..5"),
                val.span()?,
            ));
        }

        let args = CommandArgs::Time(TimeCommand { scale });
        zf_ffi::cmd_legacy(args);

        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}
