use gdnative::prelude::*;

use crate::{
    common::get_tree,
    entities::Mission,
    path::path::levels,
    vm::{
        register_vm_signal, Command, CommandInput, GameCommand, MissionCommand, VMConnecter,
        VMSignal,
    },
};

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(register_vm_signal)]
pub struct SysManager;

#[methods]
impl SysManager {
    fn new(_base: TRef<Node>) -> Self {
        SysManager
    }

    #[method]
    fn _ready(&self, #[base] base: TRef<Node>) {
        base.connect_vm_signal(VMSignal::OnCmdParsed.into());
    }

    #[method]
    fn on_cmd_parsed(&self, #[base] base: &Node, input: CommandInput) {
        let res = match &input.cmd {
            Command::Mission(m) => match m {
                MissionCommand::Info => Mission::dummy().summary(),
            },
            Command::Game(g) => match g {
                GameCommand::Start => {
                    // TODO: handle this error.
                    get_tree(base).change_scene(levels::SANDBOX).unwrap();
                    "Game started"
                }
                GameCommand::Menu => {
                    get_tree(base).change_scene(levels::START_MENU).unwrap();
                    "Game Menu"
                }
                GameCommand::End => {
                    get_tree(base).quit(0);
                    "Game End"
                }
            }
            .to_owned(),
            _ => return,
        };
        let res = input.into_result(Ok(res));
        base.emit_signal(VMSignal::OnCmdResult, &res.as_var());
    }
}
