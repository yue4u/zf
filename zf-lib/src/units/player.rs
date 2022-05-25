use gdnative::{api::PathFollow, prelude::*};

use crate::{
    common::{Position, Rotation, Vector3DisplayShort},
    ui::CommandPalette,
    vm::{Command, EngineCommand},
};

#[derive(NativeClass, Default)]
#[inherit(Node)]
pub struct Player {
    speed: f64,
    position: Position,
    rotation: Rotation,
    engine: EngineStatus,
}

#[derive(Debug)]
pub enum EngineStatus {
    On(i8),
    Off,
}

impl Default for EngineStatus {
    fn default() -> Self {
        EngineStatus::Off
    }
}

const MAX_SPEED: f64 = 1. / 30.;

#[methods]
impl Player {
    fn new(_owner: &Node) -> Self {
        godot_print!("prepare Player");
        Player::default()
    }

    pub fn path() -> &'static str {
        "/root/Scene/Game/Path/PathFollow/t-mjolnir"
    }

    #[export]
    fn _ready(&mut self, owner: TRef<Node>) -> Option<()> {
        CommandPalette::connect_on_cmd_parsed(owner)
    }

    #[export]
    fn on_cmd_parsed(&mut self, _owner: &Node, command: Command) {
        match command {
            Command::Engine(EngineCommand::Stop) => self.engine = EngineStatus::Off,
            Command::Engine(EngineCommand::Thruster(percent)) => {
                self.engine = EngineStatus::On(percent)
            }
            Command::Engine(EngineCommand::Start) => self.engine = EngineStatus::On(100),
            _ => {}
        }

        match self.engine {
            EngineStatus::On(percent) => self.speed = MAX_SPEED * percent as f64 / 100.,
            EngineStatus::Off => self.speed = 0.,
        }
    }

    #[export]
    fn _process(&mut self, owner: &Node, delta: f64) -> Option<()> {
        let transform = owner.cast::<Spatial>()?.global_transform();
        self.position = transform.origin;
        self.rotation = transform.basis.to_euler();

        (self.speed > 0.01).then_some(())?;
        godot_dbg!("{}", self.speed);

        let follow = unsafe { owner.get_parent()?.assume_safe() }.cast::<PathFollow>()?;
        follow.set_unit_offset((follow.unit_offset() + self.speed * delta).fract());
        Some(())
    }

    pub fn display(&self) -> String {
        format!(
            r#"[b]Status[/b]
speed: {:.2}
position: {}
rotation: {}

[b]Engine[/b]
engine: {:?}
"#,
            self.speed,
            self.position.display(),
            self.rotation.display(),
            self.engine
        )
    }
}
