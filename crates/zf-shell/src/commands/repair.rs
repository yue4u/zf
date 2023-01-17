use nu_protocol::{IntoPipelineData, Signature};
use zf_ffi::CommandArgs;

use crate::cmd;

cmd::proxy!(
    Repair,
    name: "repair",
    usage: "Repair our spaceship",
    arg: CommandArgs::Repair
);
