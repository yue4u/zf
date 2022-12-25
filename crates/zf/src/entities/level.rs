use zf_runtime::strip_ansi;

use crate::{common::StyledLabel, refs::path::LevelName};

// TODO: this transform can be done at compile time
fn quote(input: &str) -> String {
    let mut out = String::new();
    let mut buf = String::new();
    let mut quote = false;
    for ch in input.trim().chars() {
        match ch {
            '`' => quote = !quote,
            other if quote => buf.push(other),
            other => {
                if buf.len() > 0 {
                    out.push_str(&StyledLabel::Code.paint(&buf));
                    buf.clear();
                }
                out.push(other)
            }
        }
    }
    out
}

macro_rules! guides_and_hints {
    (
        @$(
            $level:path where
            guide: $guide:expr,
            hint: [$($x:expr),*],
        )@*
    ) => {

        fn guides(level: &LevelName) -> String {
            match level {
                $(
                    $level => quote($guide),
                )*
            }
        }

        fn hints(level: &LevelName) -> Vec<&str> {
            match level {
                $(
                    $level => vec![$($x),*],
                )*
            }
        }

        pub const LEVELS: &'static [LevelName] = &[
            $(
                $level,
            )*
        ];
    };
}

use nu_ansi_term::Color::*;
use LevelName::*;

guides_and_hints!(
    @StartMenu where
    guide: r#"
Type `tutorial` to continue or `help` for help.
"#,
    hint: [
        "tutorial"
    ],

    @TutorialEngine where
    guide: r#"
Type `help engine` to explore the engine command
"#,
    hint: [
        "engine thruster 100"
    ],

    @TutorialEngineRel where
    guide: r#"
Type `help engine rel` to explore the engine rel command

Note: you can use `alias` to create shortcut like `alias r = engine rel`
"#,
    hint: [
        "alias r = engine rel",
        "r -x -8 -y 5 -z 5",
        "r -x 7 -y 8 -z 13",
        "r -x 18 -y -3 -z 28"
    ],

    @TutorialMissionEngineRel where
    guide:
        r#"
We can use `|` to pipe data from one command to another!
For example, to show target points of current mission:
`mission targets | table`
Since mission targets returns a list, we can use get 0 to get the first one,
`mission targets | get 0 | table`
or using 0.pos as a index to get detailed info
`mission targets | get 0.pos | table`
"#,
    hint: [
        "alias m = (mission targets | get 0 | engine rel)",
        "m",
        "m",
        "m"
    ],

    @TutorialTaskMissionEngineRel where
    guide: r#"
`task` command is special, we can use `task` to create tasks that runs in the background!
For example, to run the task we created in the before mission:
`task run --every 1sec 'mission targets | get 0 | engine rel'`
Type `help task run` to explore more
"#,
    hint: ["task run --every 1sec 'mission targets | get 0 | engine rel'"],

    @TutorialTaskEngineCombine where
    guide: r#"
We can issue two commands togetter in one line seperated by `;`.
For example: `engine on; 1 + 1`.
Try combining task command and engine command togetter!
"#,
    hint: ["e t 100; task run -e 1sec 'mission targets | get 0 | engine rel'"],

    @TutorialFire where
    guide: r#"
let's use our weapon system!
`fire fm` allow use to fire a homing missile,
while enemy positions can be retrieved from `radar` command.
Combining this two commands to destroy enemies!
"#,
    hint: [
        "task run -e 1sec 'radar | get 0 | fire hm'"
    ],

    @Sandbox where
    guide: "this is a sandbox",
    hint: ["game end"],

    @Unknown where
    guide: "???",
    hint: ["game menu"],
);

pub trait LevelHelper {
    fn hint(&self) -> String;
    fn guide(&self) -> String;
}

impl LevelHelper for LevelName {
    fn hint(&self) -> String {
        hints(&self)
            .iter()
            .enumerate()
            .map(|(i, cmd)| format!("[{i}] {}", StyledLabel::Hint.paint(cmd)))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn guide(&self) -> String {
        let guide = guides(&self);
        let hint = format!("Type {} to get hint!", StyledLabel::Code.paint("hint"));
        let text = format!("{guide}\n{hint}");

        let lines = DarkGray.paint(
            "=".repeat(
                strip_ansi(&guide)
                    .lines()
                    .map(|l| l.len())
                    .max()
                    .unwrap_or_default(),
            ),
        );
        format!("{lines}\n{text}\n{lines}")
    }
}
