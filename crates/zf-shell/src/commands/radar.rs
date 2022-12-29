use nu_protocol::{IntoPipelineData, Signature};
use zf_ffi::{CommandArgs, RadarCommand};

use crate::cmd;

cmd::proxy!(
    Radar,
    name: "radar",
    usage: "Get radar result",
    arg: CommandArgs::Radar(RadarCommand{})
);
