use std::{borrow::Borrow, cell::RefCell};

use gdnative::{api::PathFollow, prelude::*};

use crate::{
    common::{self, Position, Rotation, Vector3DisplayShort},
    vm::{register_vm_signal, Command, CommandInput, EngineCommand, VMConnecter, VMSignal},
};

#[derive(NativeClass, Default)]
#[inherit(Spatial)]
#[register_with(register_vm_signal)]
pub struct Player {
    speed: RefCell<f64>,
    position: Position,
    rotation: Rotation,
    engine: RefCell<EngineStatus>,
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
        node.connect_vm_signal(VMSignal::OnCmdParsed);
        Some(())
    }

    #[export]
    fn on_cmd_parsed(&self, owner: &Spatial, input: CommandInput) -> Option<()> {
        let current_status = self.engine.borrow();
        let next_status = match &input.cmd {
            Command::Engine(EngineCommand::Off) => Some(EngineStatus::Off),
            Command::Engine(EngineCommand::Thruster(percent)) => match &*current_status {
                EngineStatus::On(_) => Some(EngineStatus::On(*percent)),
                _ => None,
            },
            Command::Engine(EngineCommand::On) => Some(EngineStatus::On(0)),
            Command::Fire(fire) => {
                godot_print!("fire: {:?}", fire);
                let missile = common::load_as::<Spatial>("res://scene/HomingMissile.tscn").unwrap();
                missile.set_scale(Vector3::new(0.05, 0.05, 0.05));
                unsafe { owner.get_node("Projectiles").unwrap().assume_safe() }
                    .add_child(missile, true);
                None
            }
            _ => None,
        }?;

        let speed = match next_status {
            EngineStatus::On(percent) => MAX_SPEED * (percent as f64) / 100.,
            EngineStatus::Off => 0.,
        };
        drop(current_status);
        godot_dbg!("???");

        self.engine.replace(next_status);
        self.speed.replace(speed);
        godot_dbg!("!!!");

        let res = input.into_result(Ok("ok".to_string()));
        owner.emit_signal(VMSignal::OnCmdResult, &res.as_var());
        Some(())
    }

    #[export]
    fn _process(&mut self, owner: &Spatial, delta: f64) -> Option<()> {
        let transform = owner.cast::<Spatial>()?.global_transform();
        self.position = transform.origin;
        self.rotation = transform.basis.to_euler();
        let speed = *self.speed.borrow();
        (speed > 0.01).then_some(())?;

        let follow = unsafe { owner.get_parent()?.assume_safe() }.cast::<PathFollow>()?;
        follow.set_unit_offset((follow.unit_offset() + speed * delta).fract());
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
            self.speed.borrow(),
            self.position.display(),
            self.rotation.display(),
            self.engine.borrow()
        )
    }
}
