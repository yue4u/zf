mod commands;
#[macro_use]
mod shell;

use commands::Hi;
use nu_command::{Math, MathSum, SplitList};
use nu_protocol::engine::{EngineState, Stack, StateWorkingSet};

fn main() {
    init_shell! {
        engine_state / stack / working_set
        Hi,
        Math,
        MathSum,
        SplitList,
    }

    let result = shell::eval(
        &mut engine_state,
        &mut stack,
        std::env::args().nth(1).unwrap_or("help".to_string()),
    );
    println!("{:?}", result);
}

#[test]
fn sanity() {
    init_shell! {
        engine_state / stack / working_set
        Math,
        MathSum,
    }

    let result = shell::eval(
        &mut engine_state,
        &mut stack,
        "[1 2 3] | math sum".to_string(),
    );
    assert_eq!(result.ok(), Some("6".to_string()));
}
