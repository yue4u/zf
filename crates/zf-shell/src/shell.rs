use anyhow::{Error, Result};
use nu_engine::eval_block;
use nu_parser::parse;
use nu_protocol::{
    engine::{EngineState, Stack, StateWorkingSet},
    CliError, PipelineData, Span,
};

macro_rules! init_shell {
    ($e:ident / $s:ident / $w:ident $( $command:expr ),* $(,)? ) => {
        let mut $e = EngineState::new();
        let mut $s = Stack::new();
        let mut $w = StateWorkingSet::new(&$e);

        $( $w.add_decl(Box::new($command)); )*

        let delta = $w.render();
        $e.merge_delta(delta).unwrap();
    };
}

fn outcome_err(
    engine_state: &EngineState,
    error: &(dyn miette::Diagnostic + Send + Sync + 'static),
) -> anyhow::Error {
    let working_set = StateWorkingSet::new(engine_state);
    Error::msg(format!("Error: {:?}", CliError(error, &working_set)))
}

pub fn eval(engine_state: &mut EngineState, stack: &mut Stack, line: String) -> Result<String> {
    let (block, delta) = {
        let mut working_set = StateWorkingSet::new(&engine_state);
        let (block, err) = parse(
            &mut working_set,
            Some(&format!("line{}", 1)),
            line.as_bytes(),
            false,
            &[],
        );

        if let Some(err) = err {
            return Err(outcome_err(&engine_state, &err));
        }
        (block, working_set.render())
    };

    if let Err(err) = engine_state.merge_delta(delta) {
        return Err(outcome_err(&engine_state, &err));
    }

    let input = PipelineData::new(Span::test_data());
    let config = engine_state.get_config();

    match eval_block(&engine_state, stack, &block, input, false, false) {
        Ok(pipeline_data) => match pipeline_data.collect_string("", config) {
            Ok(s) => return Ok(s),
            Err(err) => return Err(outcome_err(&engine_state, &err)),
        },
        Err(err) => return Err(outcome_err(&engine_state, &err)),
    }
}
