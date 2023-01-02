use std::cell::RefCell;

use gdnative::{
    api::{Area, CSGSphere, PathFollow, ShaderMaterial},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use zf_ffi::{CommandArgs, EngineCommand, ShieldCommand};

use crate::{
    common::{Position, Rotation, SceneLoader, Vector3DisplayShort},
    refs::{
        self,
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
    pub shield: PlayerShield,
    position: RefCell<Position>,
    rotation: RefCell<Rotation>,
    engine: EngineState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerShield {
    count: usize,
    #[serde(skip_serializing)]
    hit: f64,
    timeout: f64,
}

impl From<Ref<Spatial>> for Player {
    fn from(value: Ref<Spatial>) -> Self {
        Player {
            base: value,
            speed: RefCell::<f64>::default(),
            shield: PlayerShield {
                count: 3,
                hit: 0.,
                timeout: 5000.,
            },
            position: RefCell::<Position>::default(),
            rotation: RefCell::<Rotation>::default(),
            engine: EngineState::default(),
        }
    }
}

type EngineRelState = Vector3;

#[derive(Debug, Default)]
struct EngineState {
    status: EngineStatus,
    rel: EngineRelState,
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
        Player::from(base.claim())
    }

    pub fn register_signal<T: NativeClass>(builder: &ClassBuilder<T>) {
        builder.signal(VMSignal::OnCmdResult.as_str()).done();
        builder.signal(PLAYER_HIT).done();
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Spatial>) -> Option<()> {
        base.add_to_group(groups::PLAYER, false);
        // FIXME: this is a hack to get it to work.
        let node = unsafe { base.get_node_as::<Node>(".")? };
        node.connect_vm_signal(VMSignal::OnCmdParsed.into());

        self.set_shield(-1.);
        Some(())
    }

    #[method]
    fn on_cmd_parsed(&mut self, #[base] base: &Spatial, input: CommandInput) -> Option<()> {
        // tracing::debug!("{:?}",&input);
        let next_status = match &input.cmd {
            CommandArgs::Engine(EngineCommand::Off) => Some(EngineStatus::Off),
            CommandArgs::Engine(EngineCommand::Thruster(percent)) => {
                Some(EngineStatus::On(*percent))
            }
            CommandArgs::Engine(EngineCommand::On) => Some(EngineStatus::On(0)),
            CommandArgs::Engine(EngineCommand::Rel { x, y, z }) => {
                let transform = base.transform();
                let rel = Vector3::new(
                    x.unwrap_or(transform.origin.x),
                    y.unwrap_or(transform.origin.y),
                    z.unwrap_or(transform.origin.z),
                );

                self.engine.rel = rel;
                None
            }
            CommandArgs::Fire(fire) => {
                // tracing::info!("fire: {:?}", fire);
                let weapon =
                    SceneLoader::load_and_instance_as::<Spatial>(scenes::HOMING_MISSILE).unwrap();
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
            CommandArgs::Shield(ShieldCommand::On) => {
                if self.shield.count > 0 {
                    self.shield.count -= 1;
                    self.set_shield(5000.);
                }
                None
            }
            _ => None,
        }?;

        let speed = match next_status {
            EngineStatus::On(percent) => MAX_SPEED * (percent as f64) / 100.,
            EngineStatus::Off => 0.,
        };

        self.engine.status = next_status;
        self.speed.replace(speed);

        let res = input.into_result(Ok("ok".to_string()));
        base.emit_signal(VMSignal::OnCmdResult, &res.as_var());
        Some(())
    }

    fn shield(&self) -> Option<Ref<Spatial>> {
        Some(unsafe {
            self.base
                .assume_safe()
                .get_node(refs::path::player_mjolnir::SHIELD)?
                .assume_safe()
                .cast::<Spatial>()?
                .assume_shared()
        })
    }

    fn set_shield(&mut self, val: f64) {
        self.shield.timeout = val;
        if let Some(shield) = self.shield() {
            unsafe { shield.assume_safe() }.set_visible(val > 0.)
        };
    }

    fn shield_shader(&self) -> Option<Ref<ShaderMaterial>> {
        Some(unsafe {
            self.shield()?
                .assume_safe()
                .get_node(refs::path::shield::CSG_SPHERE)?
                .assume_safe()
                .cast::<CSGSphere>()?
                .material()?
                .assume_safe()
                .cast::<ShaderMaterial>()?
                .assume_shared()
        })
    }

    fn set_shield_hit(&mut self, value: f64) {
        self.shield.hit = value;
        if let Some(shield) = self.shield_shader() {
            unsafe {
                shield
                    .assume_safe()
                    .set_shader_param("hit", self.shield.hit);
            }
        }
    }

    #[method]
    fn _process(&mut self, #[base] base: &Spatial, delta: f64) -> Option<()> {
        self.set_shield_hit((self.shield.hit - delta).max(0.));

        if self.shield.timeout > 0. {
            self.shield.timeout -= delta * 1000.;
            self.set_shield(self.shield.timeout - delta)
        }

        let global_transform = base.cast::<Spatial>()?.global_transform();
        self.position.replace(global_transform.origin);
        self.rotation.replace(global_transform.basis.to_euler());

        let local_transform = base.transform();

        base.set_transform(Transform {
            basis: local_transform.basis,
            origin: local_transform
                .origin
                .linear_interpolate(self.engine.rel, delta as f32),
        });

        let speed = *self.speed.borrow();

        let follow = unsafe { base.get_parent()?.assume_safe() }.cast::<PathFollow>()?;
        follow.set_offset(follow.offset() + speed * 500. * delta);
        Some(())
    }

    #[method]
    pub fn damage(&mut self, ammount: u32) {
        if self.shield.timeout > 0. {
            self.set_shield_hit(1.);
            return;
        }

        unsafe { self.base.assume_safe() }.emit_signal(PLAYER_HIT, &[ammount.to_variant()]);
    }

    pub fn display(&self) -> String {
        format!(
            r#"[b][color=#4FFFCA]Status[/color][/b]
speed: {:.2}
position: {}
rotation: {}
shield left: {}
shield timeout: {:.2}
[b][color=#4FFFCA]Engine[/color][/b]
status: {:?}
rel: {}
"#,
            self.speed.borrow(),
            self.position.borrow().display(),
            self.rotation.borrow().display(),
            self.shield.count,
            self.shield.timeout.max(0.),
            self.engine.status,
            fmt_rel(self.engine.rel)
        )
    }
}

fn fmt_rel(Vector3 { x, y, z }: Vector3) -> String {
    format!("x: {x:.1}, y: {y:.1}, z: {z:.1}")
}
