use nu_protocol::{IntoPipelineData, Signature};
use zf_ffi::CommandArgs;

use crate::cmd;

cmd::proxy!(
    Credits,
    name: "credits",
    usage: "Open repository url for credits",
    arg: CommandArgs::Credits
);
