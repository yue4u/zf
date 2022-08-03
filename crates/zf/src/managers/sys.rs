use gdnative::prelude::*;

use crate::{
    entities::Mission,
    vm::{self, Command, CommandExecutor, CommandInput, MissionCommand},
};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct SysManager;

#[methods]
impl SysManager {
    fn new(_owner: TRef<Node>) -> Self {
        SysManager
    }

    #[export]
    fn _ready(&self, owner: TRef<Node>) {
        vm::connect_on_cmd_parsed(owner);
    }

    #[export]
    fn on_cmd_parsed(&mut self, owner: &Node, input: CommandInput) {
        let res = match &input.cmd {
            Command::Help => HELP.to_owned(),
            Command::Mission(m) => match m {
                MissionCommand::Summary => Mission::dummy().summary(),
                MissionCommand::Tartget => Mission::dummy().targets().join("\n"),
                MissionCommand::Position => format!("{:?}", Mission::dummy().positions()),
            },
            _ => return,
        };
        let res = input.into_result(Ok(res));
        owner.send_result(res);
    }
}

const HELP: &'static str = r#"ZF

help, h           Show this help
game, g
    start         Start game
    stop          Stop game
mission, m
    summary, s    Show mission summary
    target, t     Get mission targets info
    position, p   Get mission targets's positions
engine, e
    on            Start engine
    off           Stop engine
    thruster, t   Set engine thruster at <percentage>
autopilot, a
    target, t     autopilot to <target>
    orbit, o      autopilot use <orbit>
"#;
