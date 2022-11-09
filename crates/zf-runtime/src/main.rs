use anyhow::{Ok, Result};
mod memory;
mod runtime;

use memory::cmd_args_from_caller;
use runtime::{test_runtime, SHELL_PRELOAD};

fn main() -> Result<()> {
    let mut runtime = test_runtime()?;
    runtime.eval(SHELL_PRELOAD).unwrap();

    let result = runtime.eval(std::env::args().nth(1).unwrap())?;
    println!("{result}");
    Ok(())
}

#[test]
fn sanity() -> anyhow::Result<()> {
    let mut runtime = test_runtime()?;

    assert_eq!(runtime.eval("[1 2 3] | math sum").unwrap(), "6".to_string());
    Ok(())
}

#[test]
fn call() -> anyhow::Result<()> {
    use zf_bridge::{CommandBridge, GameCommand};

    let mut runtime = test_runtime()?;
    runtime.eval("game start")?;

    assert_eq!(
        runtime.store.data().ext.last_cmd_call,
        Some(CommandBridge::Game(GameCommand::Start))
    );
    Ok(())
}

#[test]
fn mystery() -> anyhow::Result<()> {
    let mut runtime = test_runtime()?;

    assert_eq!(
        runtime.eval("mystery").unwrap(),
        "🌈 it works!!".to_string()
    );
    assert_eq!(
        runtime.eval("mystery | str contains 🌈").unwrap(),
        "true".to_string()
    );

    Ok(())
}

#[test]
fn viewers() -> anyhow::Result<()> {
    let mut runtime = test_runtime()?;

    assert_eq!(
        runtime.eval("[1 2 3] | table").unwrap(),
        r#"
╭───┬───╮
│ 0 │ 1 │
│ 1 │ 2 │
│ 2 │ 3 │
╰───┴───╯"#
            .trim()
    );
    assert_eq!(
        runtime.eval("[a b c] | grid").unwrap(),
        "a │ b │ c
"
        .to_string()
    );

    Ok(())
}

#[test]
fn filters() -> anyhow::Result<()> {
    let mut runtime = test_runtime()?;

    assert_eq!(
        runtime.eval("{ project: zf } | get project").unwrap(),
        "zf".to_string()
    );
    assert_eq!(
        runtime.eval("[a b c] | grid").unwrap(),
        "a │ b │ c
"
        .to_string()
    );

    Ok(())
}

#[test]
fn preload() -> anyhow::Result<()> {
    let mut runtime = test_runtime()?;
    runtime.eval(SHELL_PRELOAD).unwrap();

    Ok(())
}
