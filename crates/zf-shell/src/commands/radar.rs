use nu_protocol::{IntoPipelineData, Signature};
use zf_bridge::{CommandBridge, RadarCommand};

use crate::cmd;

cmd::proxy!(
    Radar,
    name: "radar",
    usage: "Get radar result",
    arg: CommandBridge::Radar(RadarCommand{})
);
