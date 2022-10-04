use nu_command::{Math, MathSum, SplitList};
use nu_engine::eval_block;
use nu_parser::parse;
use nu_protocol::{
    engine::{Command, EngineState, Stack, StateWorkingSet},
    CliError, IntoPipelineData, PipelineData, ShellError, Signature, Span, Value,
};

use crate::runtime;

#[derive(Clone)]
struct Hi;

impl Command for Hi {
    fn name(&self) -> &str {
        "wasm"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("wasm")
    }

    fn usage(&self) -> &str {
        "eval wasm"
    }

    fn run(
        &self,
        _engine_state: &nu_protocol::engine::EngineState,
        _stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::ast::Call,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, ShellError> {
        let mut runtime = runtime::Runtime::new();
        let mut store = runtime.store(());

        let hello = runtime::Func::wrap(&mut store, || {
            println!("Calling back...");
            println!("> hello from wasm!");
        });

        runtime
            .run(&mut store, &[hello.into()], runtime::HELLO_WAT)
            .map_err(|_| {
                ShellError::ExternalCommand(
                    "wasm error".to_owned(),
                    "wasm error".to_owned(),
                    Span::unknown(),
                )
            })?;

        Ok(Value::String {
            val: "run!".into(),
            span: call.head,
        }
        .into_pipeline_data())
    }
}

fn outcome_err(
    engine_state: &EngineState,
    error: &(dyn miette::Diagnostic + Send + Sync + 'static),
) -> ! {
    let working_set = StateWorkingSet::new(engine_state);

    eprintln!("Error: {:?}", CliError(error, &working_set));

    std::process::exit(1);
}

pub fn eval(line: String) {
    let mut stack = Stack::new();
    // let mut engine_state = create_default_context();
    let mut engine_state = EngineState::new();
    let mut working_set = StateWorkingSet::new(&engine_state);

    working_set.add_decl(Box::new(Hi));
    working_set.add_decl(Box::new(Math));
    working_set.add_decl(Box::new(MathSum));
    working_set.add_decl(Box::new(SplitList));
    let delta = working_set.render();
    engine_state.merge_delta(delta).unwrap();

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
            outcome_err(&engine_state, &err);
        }
        (block, working_set.render())
    };

    if let Err(err) = engine_state.merge_delta(delta) {
        outcome_err(&engine_state, &err);
    }

    let input = PipelineData::new(Span::test_data());
    let config = engine_state.get_config();

    match eval_block(&engine_state, &mut stack, &block, input, false, false) {
        Ok(pipeline_data) => match pipeline_data.collect_string("", config) {
            Ok(s) => println!("result: {}", s),
            Err(err) => outcome_err(&engine_state, &err),
        },
        Err(err) => outcome_err(&engine_state, &err),
    }
}
