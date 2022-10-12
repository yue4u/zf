use nu_protocol::{IntoPipelineData, Signature};
use zf_bridge::{CommandBridge, MissionCommand};

use super::zf_call;

zf_call::proxy_command!(
    Mission,
    name: "mission",
    usage: "Get current mission info",
    arg: CommandBridge::Mission(MissionCommand::Info)

);
