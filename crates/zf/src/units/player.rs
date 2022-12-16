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
    vm::{CommandInput, VMConnecter, VMSignal},
    weapons::HomingMissile,
};

#[derive(NativeClass)]
#[inherit(Spatial)]
#[register_with(Self::register_signal)]
pub struct Player {
    #[allow(unused)]
    base: Ref<Spatial>,
    speed: RefCell<f64>,
    position: RefCell<Position>,
    rotation: RefCell<Rotation>,
    engine: RefCell<EngineStatus>,
}

impl From<Ref<Spatial>> for Player {
    fn from(value: Ref<Spatial>) -> Self {
        Player {
            base: value,
            speed: RefCell::<f64>::default(),
            position: RefCell::<Position>::default(),
            rotation: RefCell::<Rotation>::default(),
            engine: RefCell::<EngineStatus>::default(),
        }
    }
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

impl Player {
    pub fn path_from<'a>(base: &'a Node) -> String {
        let player = unsafe {
            base.get_tree()
                .unwrap()
                .assume_safe()
                .get_nodes_in_group(groups::PLAYER)
                .get(0)
                .to::<Ref<Node>>()
                .unwrap()
        };
        unsafe { player.assume_safe() }.get_path().to_string()
    }
}

pub const PLAYER_HIT: &'static str = "player_hit";

#[methods]
impl Player {
    fn new(base: TRef<Spatial>) -> Self {
        // tracing::info!("prepare Player");
        Player::from(base.claim())
    }

    pub fn register_signal<T: NativeClass>(builder: &ClassBuilder<T>) {
        builder.signal(VMSignal::OnCmdResult.as_str()).done();
        builder.signal(PLAYER_HIT).done();
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
        // tracing::debug!("{:?}",&input);
        let current_status = self.engine.borrow();
        let next_status = match &input.cmd {
            CommandArgs::Engine(EngineCommand::Off) => Some(EngineStatus::Off),
            CommandArgs::Engine(EngineCommand::Thruster(percent)) => match &*current_status {
                EngineStatus::On(_) => Some(EngineStatus::On(*percent)),
                _ => None,
            },
            CommandArgs::Engine(EngineCommand::On) => Some(EngineStatus::On(0)),
            CommandArgs::Fire(fire) => {
                // tracing::info!("fire: {:?}", fire);
                let weapon =
                    common::SceneLoader::load_and_instance_as::<Spatial>(scenes::HOMING_MISSILE)
                        .unwrap();
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

        // tracing::debug!("{:?}",&next_status);

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
        tracing::debug!("{:?}", "damage player!");
        unsafe { self.base.assume_safe() }.emit_signal(PLAYER_HIT, &[1.to_variant()]);
    }

    pub fn display(&self) -> String {
        format!(
            r#"[b][color=#4FFFCA]Status[/color][/b]
speed: {:.2}
position: {}
rotation: {}

[b][color=#4FFFCA]Engine[/color][/b]
engine: {:?}
"#,
            self.speed.borrow(),
            self.position.borrow().display(),
            self.rotation.borrow().display(),
            self.engine.borrow()
        )
    }
}
