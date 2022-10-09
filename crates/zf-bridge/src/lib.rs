pub use bincode::*;

#[derive(Decode, Encode, Debug)]
pub enum ZFCommandArgs {
    // Help,
    // Game(GameCommand),
    // Mission(MissionCommand),
    Engine(EngineCommand),
    // Autopilot(AutopilotCommand),
    // Unkonwn(String),
    // Fire(FireCommand),
    // Radar(RadarCommand),
    // UI(UICommand),
    Invalid,
}

#[derive(Decode, Encode, Debug)]
pub enum EngineCommand {
    On,
    Off,
    Thruster(i8),
}
