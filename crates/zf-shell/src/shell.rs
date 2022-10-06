use anyhow::{Error, Result};
use nu_command::{Math, MathSum};
use nu_engine::eval_block;
use nu_parser::parse;
use nu_protocol::{
    engine::{EngineState, Stack, StateWorkingSet},
    CliError, PipelineData, Span,
};

use crate::commands::Hi;

macro_rules! init_shell {
    ($e:ident / $s:ident $( $command:expr ),* $(,)? ) => {
        let mut $e = EngineState::new();
        let mut $s = Stack::new();
        let mut working_set = StateWorkingSet::new(&$e);

        $( working_set.add_decl(Box::new($command)); )*

        let delta = working_set.render();
        $e.merge_delta(delta).unwrap();
    };
}

pub fn eval(line: String) -> Result<String> {
    init_shell! {
        engine_state / stack
        Hi,
        Math,
        MathSum,
    }
    return eval_impl(
        &mut engine_state, //
        &mut stack,
        line,
    );
}

fn outcome_err(
    engine_state: &EngineState,
    error: &(dyn miette::Diagnostic + Send + Sync + 'static),
) -> anyhow::Error {
    let working_set = StateWorkingSet::new(engine_state);
    Error::msg(format!("Error: {:?}", CliError(error, &working_set)))
}

pub fn eval_impl(
    engine_state: &mut EngineState,
    stack: &mut Stack,
    line: String,
) -> Result<String> {
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
