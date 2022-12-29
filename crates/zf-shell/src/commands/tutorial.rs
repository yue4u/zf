use nu_protocol::{IntoPipelineData, Signature};
use zf_ffi::CommandArgs;

use crate::cmd;

cmd::proxy!(
    Tutorial,
    name: "tutorial",
    usage: "Start game tutorial",
    arg: CommandArgs::Tutorial
);
