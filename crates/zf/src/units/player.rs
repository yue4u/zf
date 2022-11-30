use std::cell::RefCell;

use gdnative::{
    api::{Area, PathFollow},
    prelude::*,
};
use zf_ffi::{CommandArgs, EngineCommand};

use crate::{
    common::{self, Position, Rotation, Vector3DisplayShort},
    refs::{
        groups::{self, Layer},
        path::scenes,
    },
    vm::{register_vm_signal, CommandInput, VMConnecter, VMSignal},
    weapons::HomingMissile,
};

#[derive(NativeClass, Default)]
#[inherit(Spatial)]
#[register_with(register_vm_signal)]
pub struct Player {
    speed: RefCell<f64>,
    position: RefCell<Position>,
    rotation: RefCell<Rotation>,
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
    fn new(_base: &Spatial) -> Self {
        // godot_print!("prepare Player");
        Player::default()
    }

    #[method]
    fn _ready(&self, #[base] base: TRef<Spatial>) -> Option<()> {
        base.add_to_group(groups::PLAYER, false);
        // FIXME: this is a hack to get it to work.
        let node = unsafe { base.get_node_as::<Node>(".")? };
        node.connect_vm_signal(VMSignal::OnCmdParsed.into());
        Some(())
    }

    #[method]
    fn on_cmd_parsed(&self, #[base] base: &Spatial, input: CommandInput) -> Option<()> {
        // godot_dbg!(&input);
        let current_status = self.engine.borrow();
        let next_status = match &input.cmd {
            CommandArgs::Engine(EngineCommand::Off) => Some(EngineStatus::Off),
            CommandArgs::Engine(EngineCommand::Thruster(percent)) => match &*current_status {
                EngineStatus::On(_) => Some(EngineStatus::On(*percent)),
                _ => None,
            },
            CommandArgs::Engine(EngineCommand::On) => Some(EngineStatus::On(0)),
            CommandArgs::Fire(fire) => {
                // godot_print!("fire: {:?}", fire);
                let weapon = common::load_as::<Spatial>(scenes::HOMING_MISSILE).unwrap();
                let weapon_area = unsafe { weapon.get_node_as::<Area>("Area") }.unwrap();
                Layer::PLAYER_FIRE.prepare_collision_for(weapon_area);
                let missile = weapon.cast_instance::<HomingMissile>().unwrap();

                missile
                    .map_mut(|m, _| m.target_pos = fire.pos.map(|(x, y, z)| Vector3::new(x, y, z)))
                    .unwrap();

                unsafe { base.get_node("Projectiles").unwrap().assume_safe() }
                    .add_child(missile, true);
                None
            }
            _ => None,
        }?;

        // godot_dbg!(&next_status);

        let speed = match next_status {
            EngineStatus::On(percent) => MAX_SPEED * (percent as f64) / 100.,
            EngineStatus::Off => 0.,
        };
        drop(current_status);

        self.engine.replace(next_status);
        self.speed.replace(speed);

        let res = input.into_result(Ok("ok".to_string()));
        base.emit_signal(VMSignal::OnCmdResult, &res.as_var());
        Some(())
    }

    #[method]
    fn _process(&mut self, #[base] base: &Spatial, delta: f64) -> Option<()> {
        let transform = base.cast::<Spatial>()?.global_transform();
        self.position.replace(transform.origin);
        self.rotation.replace(transform.basis.to_euler());
        let speed = *self.speed.borrow();
        (speed > 0.01).then_some(())?;

        let follow = unsafe { base.get_parent()?.assume_safe() }.cast::<PathFollow>()?;
        follow.set_unit_offset((follow.unit_offset() + speed * delta).fract());
        Some(())
    }

    #[method]
    pub fn damage(&self) {
        // godot_dbg!("damage player!");
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
            self.position.borrow().display(),
            self.rotation.borrow().display(),
            self.engine.borrow()
        )
    }
}
