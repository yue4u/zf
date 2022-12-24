use nu_protocol::{IntoPipelineData, Signature};
use zf_ffi::{CommandArgs, MissionCommand};

use crate::cmd;

cmd::proxy!(
    Mission,
    name: "mission",
    usage: "Get current mission info",
    arg: CommandArgs::Mission(MissionCommand::Info)
);

cmd::proxy!(
    MissionTargets,
    name: "mission targets",
    usage: "Get current mission targets info",
    arg: CommandArgs::Mission(MissionCommand::Targets)
);
