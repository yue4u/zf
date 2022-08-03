use gdnative::{api::PathFollow, prelude::*};

use crate::{
    common::{self, Position, Rotation, Vector3DisplayShort},
    vm::{Command, EngineCommand},
    vm_connector::{self, CommandInput},
};

#[derive(NativeClass, Default)]
#[inherit(Spatial)]
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
    fn new(_owner: &Spatial) -> Self {
        godot_print!("prepare Player");
        Player::default()
    }

    #[export]
    fn _ready(&mut self, owner: TRef<Spatial>) -> Option<()> {
        // FIXME: this is a hack to get it to work.
        let node = unsafe { owner.get_node_as::<Node>(".")? };
        vm_connector::connect_on_cmd_parsed(node)
    }

    #[export]
    fn on_cmd_parsed(&mut self, owner: &Spatial, input: CommandInput) {
        match input.cmd {
            Command::Engine(EngineCommand::Off) => self.engine = EngineStatus::Off,
            Command::Engine(EngineCommand::Thruster(percent)) => {
                if let EngineStatus::On(_) = self.engine {
                    self.engine = EngineStatus::On(percent)
                }
                // TODO: throw error if engine is off
            }
            Command::Engine(EngineCommand::On) => self.engine = EngineStatus::On(0),
            Command::Fire(fire) => {
                godot_print!("fire: {:?}", fire);
                let missile = common::load_as::<Spatial>("res://scene/HomingMissile.tscn").unwrap();
                // let global = owner.global_transform();
                // missile.set_global_transform(global);
                missile.set_scale(Vector3::new(0.05, 0.05, 0.05));
                unsafe { owner.get_node("Projectiles").unwrap().assume_safe() }
                    .add_child(missile, true);
            }
            _ => {}
        }

        match self.engine {
            EngineStatus::On(percent) => self.speed = MAX_SPEED * percent as f64 / 100.,
            EngineStatus::Off => self.speed = 0.,
        }
    }

    #[export]
    fn _process(&mut self, owner: &Spatial, delta: f64) -> Option<()> {
        let transform = owner.cast::<Spatial>()?.global_transform();
        self.position = transform.origin;
        self.rotation = transform.basis.to_euler();

        (self.speed > 0.01).then_some(())?;

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
