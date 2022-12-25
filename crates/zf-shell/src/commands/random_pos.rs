use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, Value,
};
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub(crate) struct RandomPos;

impl Command for RandomPos {
    fn name(&self) -> &str {
        "random pos"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
    }

    fn usage(&self) -> &str {
        "random pos"
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let mut rng = thread_rng();
        let mut f = || Value::Float {
            val: rng.gen_range(-10.0..=10.0),
            span: call.span(),
        };
        Ok(Value::Record {
            cols: vec!["pos".to_owned()],
            vals: vec![Value::List {
                vals: vec![f(), f(), f()],
                span: call.span(),
            }],
            span: call.span(),
        }
        .into_pipeline_data())
    }
}
