use gdnative::prelude::*;

use crate::{
    entities::Mission,
    vm::{register_vm_signal, Command, CommandInput, MissionCommand, VMConnecter, VMSignal},
};

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(register_vm_signal)]
pub struct SysManager;

#[methods]
impl SysManager {
    fn new(_owner: TRef<Node>) -> Self {
        SysManager
    }

    #[export]
    fn _ready(&self, owner: TRef<Node>) {
        owner.connect_vm_signal(VMSignal::OnCmdParsed);
    }

    #[export]
    fn on_cmd_parsed(&self, owner: &Node, input: CommandInput) {
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
        owner.emit_signal(VMSignal::OnCmdResult, &res.as_var());
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
    target, t     Autopilot to <target>
    orbit, o      Autopilot use <orbit>
radar             Show radar info
"#;
