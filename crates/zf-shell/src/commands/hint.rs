use nu_protocol::{IntoPipelineData, Signature};
use zf_ffi::CommandArgs;

use crate::cmd;

cmd::proxy!(
    Hint,
    name: "hint",
    usage: "Get level Hint",
    arg: CommandArgs::Hint
);
