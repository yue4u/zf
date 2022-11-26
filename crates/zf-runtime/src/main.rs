use anyhow::{Ok, Result};
mod memory;
mod runtime;

use memory::*;
use runtime::{test_runtime, SHELL_PRELOAD};

fn main() -> Result<()> {
    let mut runtime = test_runtime()?;
    runtime.eval(SHELL_PRELOAD).unwrap();

    let result = runtime.eval(std::env::args().nth(1).unwrap())?;
    println!("{result}");
    Ok(())
}

#[cfg(test)]
use expect_test::{expect, Expect};

#[cfg(test)]
fn check(actual: impl ToString, expect: Expect) {
    expect.assert_eq(&actual.to_string());
}

#[test]
fn sanity() -> anyhow::Result<()> {
    let mut runtime = test_runtime()?;

    check(runtime.eval("[1 2 3] | math sum").unwrap(), expect!["6"]);
    Ok(())
}

#[cfg(test)]
fn strip_ansi(input: impl std::fmt::Display) -> String {
    String::from_utf8_lossy(&strip_ansi_escapes::strip(input.to_string()).unwrap()).to_string()
}

#[test]
fn error() -> anyhow::Result<()> {
    let mut runtime = test_runtime()?;

    check(
        strip_ansi(runtime.eval("!").unwrap_err()),
        expect![[r#"
            Error: nu::shell::external_commands (link)

              × Running external commands not supported
               ╭─[line0:1:1]
             1 │ !
               · ┬
               · ╰── external not supported
               ╰────
        "#]],
    );
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

    check(runtime.eval("mystery").unwrap(), expect!["🌈 it works!!"]);

    check(
        runtime.eval("mystery | str contains 🌈").unwrap(),
        expect!["true"],
    );

    Ok(())
}

#[test]
fn viewers() -> anyhow::Result<()> {
    let mut runtime = test_runtime()?;

    check(
        runtime.eval("[1 2 3] | table").unwrap(),
        expect![[r#"
            ╭───┬───╮
            │ 0 │ 1 │
            │ 1 │ 2 │
            │ 2 │ 3 │
            ╰───┴───╯"#]],
    );
    check(
        runtime.eval("[a b c] | grid").unwrap(),
        expect![[r#"
            a │ b │ c
        "#]],
    );

    Ok(())
}

#[test]
fn filters() -> anyhow::Result<()> {
    let mut runtime = test_runtime()?;

    check(
        runtime.eval("{ project: zf } | get project").unwrap(),
        expect!["zf"],
    );
    check(
        runtime.eval("[a b c] | grid").unwrap(),
        expect![[r#"
            a │ b │ c
        "#]],
    );

    Ok(())
}

#[test]
fn preload() -> anyhow::Result<()> {
    let mut runtime = test_runtime()?;
    runtime.eval(SHELL_PRELOAD).unwrap();

    check(
        strip_ansi(runtime.eval("e --help").unwrap()),
        expect![[r#"
            engine

            Usage:
              > engine 

            Subcommands:
              engine off - Turn off engine
              engine on - Turn on engine
              engine t - Set engine thruster
              engine thruster - Set engine thruster

            Flags:
              -h, --help - Display this help message

        "#]],
    );

    Ok(())
}

#[test]
fn term_size() -> anyhow::Result<()> {
    let mut runtime = test_runtime()?;

    check(
        runtime.eval("term size | table").unwrap(),
        expect![[r#"
            ╭─────────┬────╮
            │ columns │ 80 │
            │ rows    │ 20 │
            ╰─────────┴────╯"#]],
    );

    Ok(())
}
