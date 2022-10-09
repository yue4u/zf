use anyhow::Result;
mod bridge;
mod runtime;
use bridge::cmd_args_from_caller;
use runtime::{prepare_for_test, Runtime};

fn main() -> Result<()> {
    let mut runtime = Runtime::init((), prepare_for_test)?;

    let result = runtime.eval(std::env::args().nth(1).unwrap())?;
    println!("{result}");
    Ok(())
}

#[test]
fn sanity() -> anyhow::Result<()> {
    let mut runtime = Runtime::init((), prepare_for_test)?;

    assert_eq!(runtime.eval("[1 2 3] | math sum").unwrap(), "6".to_string());
    Ok(())
}
