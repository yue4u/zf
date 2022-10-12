use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Value,
};

use crate::imports;
use zf_bridge::{CommandBridge, FireCommand};

use super::expect_flag;

#[derive(Clone)]
pub(crate) struct Fire;

impl Command for Fire {
    fn name(&self) -> &str {
        "fire"
    }

    fn signature(&self) -> Signature {
        Signature::build("fire")
            .named("weapon", SyntaxShape::String, "weapon name", Some('w'))
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
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let args = CommandBridge::Fire(FireCommand {
            weapon: expect_flag(engine_state, stack, call, "weapon")?,
            target: expect_flag(engine_state, stack, call, "target")?,
        });
        imports::zf_call(args);
        // TODO: we may want to return true/false from here
        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}
