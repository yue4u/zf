use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Value,
};

use crate::imports;
use zf_bridge::{CommandBridge, UIAction, UICommand};

#[derive(Clone)]
pub(crate) struct UI;

impl Command for UI {
    fn name(&self) -> &str {
        "ui"
    }

    fn signature(&self) -> Signature {
        Signature::build("ui")
            .required("action", SyntaxShape::String, "ui label")
            .named("label", SyntaxShape::String, "ui label", Some('l'))
    }

    fn usage(&self) -> &str {
        "ui"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let (action_val, action_span) = match call.req::<Value>(engine_state, stack, 0)? {
            Value::String { val, span } => (val, span),
            _ => {
                return Ok(Value::Nothing { span: call.head }.into_pipeline_data());
            }
        };

        let args = CommandBridge::UI(UICommand {
            action: match action_val.as_str() {
                "show" => UIAction::Show,
                "hide" => UIAction::Hide,
                unknown => {
                    return Err(ShellError::IncompatibleParametersSingle(
                        format!(
                            "Unknown action `{}`, `show` and `hide` are available",
                            unknown
                        ),
                        action_span,
                    ))
                }
            },
            label: match call.get_flag(engine_state, stack, "label")? {
                Some(val) => val,
                _ => {
                    return Err(ShellError::MissingParameter(
                        "label not exist".to_string(),
                        call.head,
                    ))
                }
            },
        });
        imports::zf_call(args);
        // TODO: we may want to return true/false from here
        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}
