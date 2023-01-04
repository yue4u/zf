use crate::runtime::{test_runtime, SHELL_PRELOAD};

#[cfg(test)]
use crate::strip_ansi;
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
    use zf_ffi::{CommandArgs, GameCommand};

    let mut runtime = test_runtime()?;
    runtime.eval("game start")?;

    assert_eq!(
        runtime.store.data().ext.last_cmd_call,
        Some(CommandArgs::Game(GameCommand::Start))
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
            ╰───┴───╯
        "#]],
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
              engine rel - Set relative pos from orbit
              engine t - Set engine thruster
              engine thruster - Set engine thruster

            Flags:
              -h, --help - Display the help message for this command

        "#]],
    );

    Ok(())
}

#[test]
fn term_size() -> anyhow::Result<()> {
    let mut runtime = test_runtime()?;
    check(
        runtime.eval("term size").unwrap(),
        expect![[r#"
            ╭─────────┬────╮
            │ columns │ 80 │
            │ rows    │ 20 │
            ╰─────────┴────╯"#]],
    );

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

#[test]
fn fsays() -> anyhow::Result<()> {
    let mut runtime = test_runtime()?;

    check(
        runtime.eval("fsays hi").unwrap(),
        expect![[r#"
             ____
            < hi >
             ----
                    \
                     \
                        _~^~^~_
                    \) /  o o  \ (/
                      '_   -   _'
                      / '-----' \
        "#]],
    );

    check(
        runtime
            .eval("echo 'Hello fellow Rustaceans!' | fsays")
            .unwrap(),
        expect![[r#"
             __________________________
            < Hello fellow Rustaceans! >
             --------------------------
                    \
                     \
                        _~^~^~_
                    \) /  o o  \ (/
                      '_   -   _'
                      / '-----' \
        "#]],
    );

    Ok(())
}

#[test]
fn cmds_len() {
    let mut runtime = test_runtime().unwrap();
    check(runtime.cmds_available().unwrap().join("\n"), expect![[r#"
        alias
        all
        any
        append
        ast
        char
        clear
        collect
        columns
        commandline
        compact
        date
        date format
        date humanize
        date list-timezone
        date now
        date to-record
        date to-table
        date to-timezone
        debug
        decode
        decode base64
        def
        def-env
        default
        describe
        detect columns
        do
        drop
        drop column
        drop nth
        each
        each while
        echo
        encode
        encode base64
        engine
        engine off
        engine on
        engine rel
        engine t
        engine thruster
        error make
        every
        export
        export alias
        export def
        export def-env
        export extern
        export use
        extern
        find
        fire
        first
        flatten
        for
        format
        format filesize
        from
        from json
        msg
        game
        game end
        game menu
        game start
        get
        grid
        group
        group-by
        headers
        help
        hi
        hide
        hide-env
        hint
        if
        ignore
        insert
        is-empty
        last
        length
        let
        level
        level ls
        level next
        level restart
        level start
        lines
        math
        math abs
        math avg
        math ceil
        math eval
        math floor
        math max
        math median
        math min
        math mode
        math product
        math round
        math sqrt
        math stddev
        math sum
        math variance
        merge
        metadata
        mission
        mission targets
        module
        move
        mystery
        overlay
        overlay hide
        overlay list
        overlay new
        overlay use
        par-each
        parse
        prepend
        radar
        random
        random bool
        random chars
        random decimal
        random dice
        random integer
        random pos
        random uuid
        range
        reduce
        reject
        rename
        reverse
        roll
        roll down
        roll left
        roll right
        roll up
        rotate
        select
        shield
        shield off
        shield on
        shuffle
        size
        skip
        skip until
        skip while
        sleep
        sort
        sort-by
        split
        split chars
        split column
        split list
        split row
        split words
        split-by
        str
        str camel-case
        str capitalize
        str collect
        str contains
        str distance
        str downcase
        str ends-with
        str index-of
        str join
        str kebab-case
        str length
        str lpad
        str pascal-case
        str replace
        str reverse
        str rpad
        str screaming-snake-case
        str snake-case
        str starts-with
        str substring
        str title-case
        str trim
        str upcase
        table
        take
        take until
        take while
        task
        task run
        task stop
        term opacity
        term size
        time
        time
        transpose
        tutorial
        ui
        uniq
        update
        update cells
        upsert
        use
        where
        window
        wrap
        zip"#]])
}
