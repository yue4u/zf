use nu_protocol::{IntoPipelineData, Signature};
use zf_ffi::{CommandArgs, GameCommand};

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
    arg: CommandArgs::Game(GameCommand::Start)
);

cmd::proxy!(
    GameMenu,
    name: "game menu",
    usage: "Goto game game",
    arg: CommandArgs::Game(GameCommand::Menu)
);

cmd::proxy!(
    GameTutorial,
    name: "game tutorial",
    usage: "Start game tutorial",
    arg: CommandArgs::Game(GameCommand::Tutorial)
);

cmd::proxy!(
    GameEnd,
    name: "game end",
    usage: "End game",
    arg: CommandArgs::Game(GameCommand::End)
);
