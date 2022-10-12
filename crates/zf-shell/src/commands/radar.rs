use nu_protocol::{IntoPipelineData, Signature};
use zf_bridge::{CommandBridge, RadarCommand};

use super::zf_call;

zf_call::proxy_command!(
    Radar,
    name: "radar",
    usage: "Get radar result",
    arg: CommandBridge::Radar(RadarCommand{})
);
