use nu_protocol::{IntoPipelineData, Signature};
use zf_bridge::{CommandBridge, GameCommand};

use super::zf_call;

zf_call::empty_command!(
    Game,
    name: "game",
    usage: "game"
);

zf_call::proxy_command!(
    GameStart,
    name: "game start",
    usage: "Start game",
    arg: CommandBridge::Game(GameCommand::Start)
);

zf_call::proxy_command!(
    GameMenu,
    name: "game menu",
    usage: "Goto game game",
    arg: CommandBridge::Game(GameCommand::Menu)
);

zf_call::proxy_command!(
    GameEnd,
    name: "game end",
    usage: "End game",
    arg: CommandBridge::Game(GameCommand::End)
);
