use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, Value,
};

#[derive(Clone)]
pub(crate) struct SpecialThanks;

impl Command for SpecialThanks {
    fn name(&self) -> &str {
        "special-thanks"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
    }

    fn usage(&self) -> &str {
        "special-thanks"
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        Ok(Value::String {
            val: r#"
戌亥とこ
リゼ・ヘルエスタ
兎田ぺこら
名取さな
南登かなる
"#
            .trim()
            .into(),
            span: call.head,
        }
        .into_pipeline_data())
    }
}
