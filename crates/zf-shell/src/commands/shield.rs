use nu_protocol::{IntoPipelineData, Signature};
use zf_ffi::{CommandArgs, ShieldCommand};

use crate::cmd;

cmd::proxy!(
    Shield,
    name: "shield",
    usage: "Show shield status",
    arg: CommandArgs::Shield(ShieldCommand::Show)
);

cmd::proxy!(
    ShieldOn,
    name: "shield on",
    usage: "Turn on shield",
    arg: CommandArgs::Shield(ShieldCommand::On)
);

cmd::proxy!(
    ShieldOff,
    name: "shield off",
    usage: "Turn off shield",
    arg: CommandArgs::Shield(ShieldCommand::Off)
);
