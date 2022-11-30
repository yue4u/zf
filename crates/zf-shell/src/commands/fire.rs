use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Value,
};

use zf_ffi::{CommandArgs, FireCommand};

#[derive(Clone)]
pub(crate) struct Fire;

impl Command for Fire {
    fn name(&self) -> &str {
        "fire"
    }

    fn signature(&self) -> Signature {
        Signature::build("fire")
            .required("weapon", SyntaxShape::String, "weapon name")
            .named("target", SyntaxShape::String, "target name", Some('t'))
    }

    fn usage(&self) -> &str {
        "fire a weapon"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let value = input.into_value(call.head);
        let mut target: Option<String> = None;
        let mut pos: Option<(f32, f32, f32)> = None;
        if let Ok((cols, vals)) = value.as_record() {
            // TODO: skip iter if both found or use a hashmap
            for (col, val) in cols.iter().zip(vals.iter()) {
                match col.as_str() {
                    "name" => {
                        target = Some(val.as_string()?);
                    }
                    "pos" => {
                        let list = val.as_list()?;
                        pos = Some((
                            list[0].as_f64()? as f32,
                            list[1].as_f64()? as f32,
                            list[2].as_f64()? as f32,
                        ));
                    }
                    _ => {}
                }
            }
        }
        let args = CommandArgs::Fire(FireCommand {
            weapon: call.req::<String>(engine_state, stack, 0)?,
            target,
            pos,
        });
        zf_ffi::cmd(args);
        // TODO: we may want to return true/false from here
        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}
