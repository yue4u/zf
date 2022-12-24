use zf_runtime::strip_ansi;

use crate::{common::StyledLabel, entities::GLOBAL_GAME_STATE, managers::VMManager};

use self::path::LevelName;
// use crate::ui::CommandPalette;

pub mod groups;
pub mod path;

pub trait HasPath {
    fn path() -> &'static str;
}

#[macro_export]
macro_rules! bind_path {
    (
        $(
            $x:ty => $p1:ident::$p2:ident
        ),*
    ) => {
        $(
            impl HasPath for $x {
                fn path() -> &'static str {
                    crate::refs::path::$p1::$p2
                }
            }
        )*
    };
}

bind_path!(
    VMManager => auto_load::VM
    // CommandPalette => base::LINE_EDIT
);

pub const LEVELS: &'static [LevelName] = &[
    LevelName::TutorialEngine,
    LevelName::TutorialEngineRel,
    LevelName::TutorialMissionEngineRel,
    LevelName::TutorialTaskMissionEngineRel,
    LevelName::TutorialTaskEngineCombine,
    LevelName::TutorialFire,
    LevelName::Sandbox,
];

pub fn next_level(current: String) -> Option<LevelName> {
    let current_idx = LEVELS.iter().position(|l| l.as_str() == &current)?;
    let next = LEVELS.get(current_idx + 1)?;
    Some(next.clone())
}

pub trait LevelHelper {
    fn hint(&self) -> String;
    fn guide(&self) -> String;
}

impl LevelHelper for LevelName {
    fn hint(&self) -> String {
        let cmds: Vec<&str> = match &self {
            LevelName::StartMenu => todo!(),
            LevelName::Sandbox => todo!(),
            LevelName::TutorialEngine => vec!["engine thruster 100"],
            LevelName::TutorialFire => todo!(),
            LevelName::TutorialEngineRel => vec![
                "alias r = engine rel",
                "r -x -8 -y 5 -z 5",
                "r -x 7 -y 8 -z 13",
                "r -x 18 -y -3 -z 28",
            ],
            LevelName::TutorialMissionEngineRel => vec![
                "alias m = (mission targets | get 0 | engine rel)",
                "m",
                "m",
                "m",
            ],
            LevelName::TutorialTaskMissionEngineRel => {
                vec!["task run --every 1sec 'mission targets | get 0 | engine rel'"]
            }
            LevelName::TutorialTaskEngineCombine => todo!(),
            LevelName::Unknown => todo!(),
            LevelName::TutorialComplete => todo!(),
        };
        cmds.iter()
            .enumerate()
            .map(|(i, cmd)| format!("[{i}] {}", StyledLabel::Hint.paint(cmd)))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn guide(&self) -> String {
        use nu_ansi_term::Color::*;

        let mission = format!("{}: Reach all target points!", Cyan.paint("Mission"),);
        let hint = format!("Type {} to get hint!", StyledLabel::Code.paint("hint"));

        let guide = match &self {
            LevelName::StartMenu => format!(
                "Type {} to continue or {} for help.",
                StyledLabel::Code.paint(
                    GLOBAL_GAME_STATE
                        .lock()
                        .map(|state| {
                            if state.tutorial_completed {
                                "game start"
                            } else {
                                "tutorial"
                            }
                        })
                        .unwrap_or_else(|_| "tutorial")
                ),
                StyledLabel::Code.paint("help")
            ),
            LevelName::Sandbox => todo!(),
            LevelName::TutorialEngine => format!(
                r#"{mission}

Type {} to explore the engine command"#,
                StyledLabel::Code.paint("engine --help"),
            ),
            LevelName::TutorialEngineRel => format!(
                r#"{mission}

Type {} to explore the engine rel command

Note: you can use {} to create shortcut like {}
"#,
                StyledLabel::Code.paint("engine rel --help"),
                StyledLabel::Code.paint("alias"),
                StyledLabel::Code.paint("alias r = engine rel"),
            ),
            LevelName::TutorialMissionEngineRel => format!(
                r#"{mission}

We can use {} to `pipe` data from one command to another!
For example, to show target points of current mission:
{}
Since `mission targets` returns a list, we can use get 0 to get the first one,
{}
or using 0.pos as a index to get detailed info
{}
"#,
                StyledLabel::Code.paint("|"),
                StyledLabel::Code.paint("mission targets | table"),
                StyledLabel::Code.paint("mission targets | get 0 | table"),
                StyledLabel::Code.paint("mission targets | get 0.pos | table"),
            ),
            LevelName::TutorialTaskMissionEngineRel => format!(
                r#"{mission}

{} command is special, we can use {} to create tasks that runs in the background!
For example, to run the task we created in the before mission:
{}
Type {} to explore more
"#,
                StyledLabel::Code.paint("task"),
                StyledLabel::Code.paint("task"),
                StyledLabel::Code
                    .paint("task run --every 1sec 'mission targets | get 0 | engine rel'"),
                StyledLabel::Code.paint("task run --help"),
            ),
            LevelName::TutorialTaskEngineCombine => todo!(),
            LevelName::TutorialFire => todo!(),
            LevelName::Unknown => todo!(),
            LevelName::TutorialComplete => todo!(),
        };

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
