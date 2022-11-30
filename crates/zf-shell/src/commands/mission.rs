use nu_protocol::{IntoPipelineData, Signature};
use zf_bridge::{CommandBridge, MissionCommand};

use crate::cmd;

cmd::proxy!(
    Mission,
    name: "mission",
    usage: "Get current mission info",
    arg: CommandBridge::Mission(MissionCommand::Info)

);
