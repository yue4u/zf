use nu_protocol::{IntoPipelineData, Signature};
use zf_ffi::{CommandArgs, ShieldCommand};

use crate::cmd;

cmd::proxy!(
    ShieldOn,
    name: "shield on",
    usage: "Turn on the shield",
    arg: CommandArgs::Shield(ShieldCommand::On)
);
