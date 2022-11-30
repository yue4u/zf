use nu_protocol::{IntoPipelineData, Signature};
use zf_ffi::{CommandArgs, MissionCommand};

use crate::cmd;

cmd::proxy!(
    Mission,
    name: "mission",
    usage: "Get current mission info",
    arg: CommandArgs::Mission(MissionCommand::Info)

);
