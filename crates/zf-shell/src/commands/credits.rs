use nu_protocol::{IntoPipelineData, Signature};
use zf_ffi::CommandArgs;

use crate::cmd;

cmd::proxy!(
    Credits,
    name: "credits",
    usage: "Get radar result",
    arg: CommandArgs::Credits
);
