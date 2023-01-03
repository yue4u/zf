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

    if buf.len() > 0 {
        out.push_str(&StyledLabel::Code.paint(&buf));
        buf.clear();
    }

    out
}

macro_rules! levels {
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

levels!(
    @Unknown where
    guide: "???",
    hint: ["???"],

    @StartMenu where
    guide: r#"
Type `tutorial` to continue or `help` for help.
"#,
    hint: [
        "tutorial"
    ],

    @TutorialHelloWorld where
    guide: r#"
Welcome to ZF! In this game we type commands to complete tasks!
Try type `hi` and hit enter to continue!
"#,
    hint: [
        "hi"
    ],

    @TutorialEngine where
    guide: r#"
Let's control user spaceship.
Type `help engine` to explore the engine command,
setting the engine thruster and reach all target points!
"#,
    hint: [
        "engine thruster 100"
    ],

    @TutorialEngineRel where
    guide: r#"
We can use `engine rel` to move relative from the orbit!
Type `help engine rel` to explore the engine rel command
"#,
    hint: [
        "engine rel -x 21 -y 8 -z 30"
    ],

    @ChallengeEngineRel where
    guide: r#"
Note: you can use `alias` to create shortcut like `alias r = engine rel`
"#,
    hint: [
        "alias r = engine rel",
        "r -x -8 -y 5 -z 5",
        "r -z 13",
        "r -x 0",
        "r -y -3 -z 20"
    ],

    @TutorialMissionEngineRel where
    guide:
        r#"
We can use `|` to pipe data from one command to another!
For example, to show target points of current mission:
`mission targets`
Since mission targets returns a list, we can use get 0 to get the first one,
`mission targets | get 0`
or using 0.pos as a index to get detailed info
`mission targets | get 0.pos`.
Try combine with `engine rel` to clear this level.
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
`task run 'random pos | engine rel'`
Type `help task` and `help task run` to explore more
"#,
    hint: ["task run 'mission targets | get 0 | engine rel'"],

    @TutorialTaskEngineCombine where
    guide: r#"
We can issue two commands togetter in one line seperated by `;`.
For example: `engine t 50; engine rel -x 5`.
Try combining task command and engine command togetter!
"#,
    hint: ["e t 100; task run 'mission targets | get 0 | e rel'"],

    @ChallengeTaskEngineCombine where
    guide: r#"Reach all target points!
You can use the `time scale` command to control the game speed"#,
    hint: ["time scale 1.5; e t 100; task run 'mission targets | get 0 | e rel'"],

    @TutorialFire where
    guide: r#"
let's use our weapon system!
`fire hm` allow use to fire a homing missile,
while enemy positions can be retrieved from `radar` command.
Combining this two commands to destroy enemies!
"#,
    hint: [
        "task run 'radar | get 0 | fire hm'"
    ],

    @TutorialShield where
    guide: r#"
Enemy appears!
Try `shield on` to avoid taking damage.
Note: shield have a total time limit, use `shield off` to turn off if not necessary
Check status on the right side or use the `shield` command.
"#,
    hint: [
        "shield on"
    ],

    @ChallengeShield where
    guide: r#"
Survive for 15 seconds!
"#,
    hint: [
        "shield on"
    ],

    @ChallengeInfinite where
    guide: "Keep alive as long as possible",
    hint: [],
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
