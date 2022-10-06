use anyhow::Result;
use nu_command::{Math, MathSum};
use nu_engine::eval_block;
use nu_parser::parse;
use nu_protocol::{
    engine::{EngineState, Stack, StateWorkingSet},
    CliError, PipelineData, Span,
};

use crate::commands::Hi;

macro_rules! eval {
    ($line:ident with $( $command:expr ),* $(,)? ) => {
        let mut engine_state = EngineState::new();
        let mut stack = Stack::new();
        let mut working_set = StateWorkingSet::new(&engine_state);

        $( working_set.add_decl(Box::new($command)); )*

        let delta = working_set.render();
        engine_state.merge_delta(delta).unwrap();

        eval_impl(
            &mut engine_state, //
            &mut stack,
            $line,
        )
    };
}

pub fn eval(line: String) -> Result<String> {
    eval! {
        line with
        Hi,
        Math,
        MathSum,
    }
}

trait CheckOutcome<T, E>
where
    E: miette::Diagnostic + Send + Sync + 'static,
{
    fn check_outcome(self, engine_state: &EngineState) -> anyhow::Result<T>;
}

impl<T, E> CheckOutcome<T, E> for Result<T, E>
where
    E: miette::Diagnostic + Send + Sync + 'static,
{
    fn check_outcome(self, engine_state: &EngineState) -> anyhow::Result<T> {
        match self {
            Ok(t) => Ok(t),
            Err(err) => {
                let working_set = StateWorkingSet::new(engine_state);
                Err(anyhow::Error::msg(format!(
                    "Error: {:?}",
                    CliError(&err, &working_set)
                )))
            }
        }
    }
}

pub fn eval_impl(
    engine_state: &mut EngineState,
    stack: &mut Stack,
    source_lines: String,
) -> Result<String> {
    let mut last_output = String::new();

    for (i, line) in source_lines.lines().enumerate() {
        let mut working_set = StateWorkingSet::new(&engine_state);
        let (block, err) = parse(
            &mut working_set,
            Some(&format!("line{}", i)),
            line.as_bytes(),
            false,
            &[],
        );

        if let Some(err) = err {
            Err(err).check_outcome(engine_state)?
        }

        let delta = working_set.render();

        engine_state
            .merge_delta(delta)
            .check_outcome(engine_state)?;

        let input = PipelineData::new(Span::test_data());
        let config = engine_state.get_config();

        last_output = eval_block(
            &engine_state, //
            stack,
            &block,
            input,
            false,
            false,
        )
        .check_outcome(engine_state)?
        .collect_string("", config)
        .check_outcome(engine_state)?;
    }
    Ok(last_output)
}
