use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Value,
};

use zf_ffi::{CommandArgs, FireCommand, WeaponName};

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

        let weapon = match call.req::<String>(engine_state, stack, 0)?.as_str() {
            "b" | "beam" => {
                if pos.is_some() {
                    return Err(ShellError::IncompatibleParametersSingle(
                        "beam does not accept setting a target pos".to_owned(),
                        call.positional_nth(0).unwrap().span,
                    ));
                }
                WeaponName::Beam
            }
            "hm" | "homing-missile" => WeaponName::HomingMissile,
            other => {
                return Err(ShellError::IncompatibleParametersSingle(
                    format!("unknown weapon: {other}"),
                    call.positional_nth(0).unwrap().span,
                ))
            }
        };

        let args = CommandArgs::Fire(FireCommand {
            weapon,
            target,
            pos,
        });
        zf_ffi::cmd_legacy(args);
        // TODO: we may want to return true/false from here
        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}
