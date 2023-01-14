use gdnative::{
    api::{Area, CSGSphere, Particles, ParticlesMaterial, PathFollow, ShaderMaterial},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use zf_ffi::{CommandArgs, EngineCommand, ShieldCommand};

use crate::{
    common::{Position, Rotation, SceneLoader, Vector3DisplayShort},
    refs::{
        self,
        groups::{self, Layer},
        path::{player_mjolnir, scenes},
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
    speed: f64,
    shield: PlayerShield,
    position: Position,
    rotation: Rotation,
    engine: EngineTargetState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerShield {
    #[serde(skip_serializing)]
    hit: f64,
    on: bool,
    time_left: f64,
}

impl From<Ref<Spatial>> for Player {
    fn from(value: Ref<Spatial>) -> Self {
        Player {
            base: value,
            speed: f64::default(),
            shield: PlayerShield {
                hit: 0.,
                on: false,
                time_left: 15.,
            },
            position: Position::default(),
            rotation: Rotation::default(),
            engine: EngineTargetState::default(),
        }
    }
}

#[derive(Debug, Default)]
struct EngineTargetState {
    thruster: EngineThruster,
    rotation: Vector3,
    rel: Vector3,
}

#[derive(Debug)]
pub enum EngineThruster {
    On(i8),
    Off,
}

impl Default for EngineThruster {
    fn default() -> Self {
        EngineThruster::Off
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

    pub fn shield(&self) -> &PlayerShield {
        &self.shield
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Spatial>) -> Option<()> {
        base.add_to_group(groups::PLAYER, false);

        // HACK: base rotation from scene, better do this inside the player scene
        self.engine.rotation.y = 180.;

        self.update_engine();
        // FIXME: this is a hack to get it to work.
        let node = unsafe { base.get_node_as::<Node>(".")? };
        node.connect_vm_signal(VMSignal::OnCmdParsed.into());
        Some(())
    }

    #[method]
    fn on_cmd_parsed(&mut self, #[base] base: &Spatial, input: CommandInput) -> Option<()> {
        // tracing::debug!("{:?}",&input);
        let next_status = match &input.cmd {
            CommandArgs::Engine(EngineCommand::Off) => Some(EngineThruster::Off),
            CommandArgs::Engine(EngineCommand::Thruster(percent)) => {
                Some(EngineThruster::On(*percent))
            }
            CommandArgs::Engine(EngineCommand::On) => Some(EngineThruster::On(0)),
            CommandArgs::Engine(EngineCommand::Rotate(rotate)) => {
                self.engine.rotation.z = *rotate;
                None
            }
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
            CommandArgs::Shield(shield @ (ShieldCommand::On | ShieldCommand::Off)) => {
                let shield_ref = unsafe { self.shield_ref().unwrap().assume_safe() };
                match shield {
                    ShieldCommand::On if self.shield.time_left > 0. => {
                        self.shield.on = true;
                        shield_ref.set_visible(true);
                    }
                    ShieldCommand::Off => {
                        self.shield.on = false;
                        shield_ref.set_visible(false);
                    }
                    _ => {}
                }
                None
            }
            _ => None,
        }?;

        let speed = match next_status {
            EngineThruster::On(percent) => MAX_SPEED * (percent as f64) / 100.,
            _ => 0.,
        };

        self.engine.thruster = next_status;
        self.update_engine();
        self.speed = speed;

        let res = input.into_result(Ok("ok".to_string()));
        base.emit_signal(VMSignal::OnCmdResult, &res.as_var());
        Some(())
    }

    fn shield_ref(&self) -> Option<Ref<Spatial>> {
        Some(unsafe {
            self.base
                .assume_safe()
                .get_node(player_mjolnir::SHIELD)?
                .assume_safe()
                .cast::<Spatial>()?
                .assume_shared()
        })
    }

    fn shield_shader(&self) -> Option<Ref<ShaderMaterial>> {
        Some(unsafe {
            self.shield_ref()?
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

    fn set_shield_time_left(&mut self, val: f64) {
        self.shield.time_left = val;
        self.shield.on = val > 0.;
        if let Some(shield) = self.shield_ref() {
            unsafe { shield.assume_safe() }.set_visible(self.shield.on)
        };
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

    fn is_shield_on(&self) -> bool {
        self.shield.on && self.shield.time_left > 0.
    }

    #[method]
    fn _process(&mut self, #[base] base: &Spatial, delta: f64) -> Option<()> {
        if self.is_shield_on() {
            self.set_shield_time_left(self.shield.time_left - delta);
            self.set_shield_hit((self.shield.hit - delta).max(0.));
        }

        let global_transform = base.cast::<Spatial>()?.global_transform();
        self.position = global_transform.origin;
        self.rotation = global_transform.basis.to_euler();

        let local_transform = base.transform();

        if self.engine.rotation.z != 0. {
            let mut local_rotation = base.rotation_degrees();
            if (self.engine.rotation.z - local_rotation.z).abs() > 2. {
                local_rotation.z += 10. * (delta as f32);
            } else {
                local_rotation.z -= 10. * (delta as f32);
            }
            base.set_rotation_degrees(local_rotation);
        }

        base.set_transform(Transform {
            basis: local_transform.basis,
            origin: local_transform
                .origin
                .linear_interpolate(self.engine.rel, delta as f32),
        });

        let follow = unsafe { base.get_parent()?.assume_safe() }.cast::<PathFollow>()?;
        follow.set_offset(follow.offset() + self.speed * 500. * delta);
        Some(())
    }

    fn update_engine(&self) {
        let amount = match self.engine.thruster {
            EngineThruster::On(amount) => amount,
            _ => 0,
        };

        [
            player_mjolnir::EL_1,
            player_mjolnir::EL_2,
            player_mjolnir::EL_3,
            player_mjolnir::EL_4,
        ]
        .into_iter()
        .for_each(|path| {
            unsafe {
                self.base
                    .assume_safe()
                    .get_node_as::<Particles>(path)
                    .map(|p| {
                        p.set_visible(amount > 0);
                        p.process_material()
                            .unwrap()
                            .assume_safe()
                            .cast::<ParticlesMaterial>()
                            .unwrap()
                            .set_initial_velocity(-10. * amount as f64 / 128.);
                        // set_amount: Amount of particles cannot be smaller than 1.
                        p.set_amount(amount.max(1) as i64);
                    })
            };
        });
    }

    #[method]
    pub fn damage(&mut self, ammount: u32) {
        if self.is_shield_on() {
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
shield timeout: {:.2}
[b][color=#4FFFCA]Engine[/color][/b]
status: {:?}
rel: {}
"#,
            self.speed,
            self.position.display(),
            self.rotation.display(),
            self.shield.time_left.max(0.),
            self.engine.thruster,
            self.engine.rel.display()
        )
    }
}
