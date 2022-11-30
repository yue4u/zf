use nu_protocol::{IntoPipelineData, Signature};
use zf_bridge::{CommandBridge, GameCommand};

use crate::cmd;

cmd::empty!(
    Game,
    name: "game",
    usage: "game"
);

cmd::proxy!(
    GameStart,
    name: "game start",
    usage: "Start game",
    arg: CommandBridge::Game(GameCommand::Start)
);

cmd::proxy!(
    GameMenu,
    name: "game menu",
    usage: "Goto game game",
    arg: CommandBridge::Game(GameCommand::Menu)
);

cmd::proxy!(
    GameEnd,
    name: "game end",
    usage: "End game",
    arg: CommandBridge::Game(GameCommand::End)
);
