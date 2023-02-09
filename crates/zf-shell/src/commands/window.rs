use nu_protocol::{IntoPipelineData, Signature};
use zf_ffi::{CommandArgs, WindowCommand};

use crate::cmd;

cmd::proxy!(
    WindowFullScreen,
    name: "game window fullscreen",
    usage: "set window mode to fullscreen",
    arg: CommandArgs::Window(WindowCommand::FullScreen)
);

cmd::proxy!(
    WindowModeWindowed,
    name: "game window windowed",
    usage: "set window mode to windowed",
    arg: CommandArgs::Window(WindowCommand::Windowed)
);
