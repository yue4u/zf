use std::{cell::RefCell, collections::HashMap};

use gdnative::{
    api::{Area, TextureRect},
    prelude::*,
};

use crate::{
    refs::{groups, HasPath},
    units::player::Player,
    vm::{register_vm_signal, Command, CommandInput, UIAction, UICommand, VMConnecter, VMSignal},
};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(NativeClass, Default)]
#[inherit(Node)]
#[register_with(register_vm_signal)]
// https://docs.rs/gdnative/latest/gdnative/export/user_data/index.html#choosing-a-wrapper-type
#[user_data(gdnative::export::user_data::MutexData<Radar>)]
pub struct Radar {
    detected: RefCell<HashMap<GodotString, Ref<Area>>>,
}

#[derive(Serialize, Deserialize)]
struct AreaInfo {
    name: String,
    pos: [f32; 3],
}

#[derive(Serialize, Deserialize)]
struct RadarResult(Vec<AreaInfo>);

#[methods]
impl Radar {
    fn new(_base: &Node) -> Self {
        Default::default()
    }

    #[method]
    fn _ready(&self, #[base] base: TRef<Node>) -> Option<()> {
        base.connect_vm_signal(VMSignal::OnCmdParsed.into());
        base.add_to_group(groups::RADAR, false);
        let player_radar = unsafe {
            base.get_node(&format!("{}/{}", Player::path(), "RadarArea"))?
                .assume_safe()
        }
        .cast::<Area>()?;

        player_radar
            .connect(
                "area_entered",
                base,
                "on_detected",
                VariantArray::new_shared(),
                0,
            )
            .expect("failed to connect area_entered");

        player_radar
            .connect(
                "area_exited",
                base,
                "on_lost",
                VariantArray::new_shared(),
                0,
            )
            .expect("failed to connect area_exited");

        Some(())
    }

    #[method]
    fn detected(&self) -> String {
        let mut radar_result = RadarResult(Vec::new());
        self.detected.borrow().iter().for_each(|(name, area)| {
            let pos = unsafe { area.assume_safe() }.global_transform().origin;
            radar_result.0.push(AreaInfo {
                name: name.to_string(),
                pos: [pos.x, pos.y, pos.z],
            });
        });
        serde_json::to_string(&radar_result).unwrap()
    }

    #[method]
    fn on_cmd_parsed(&self, #[base] base: TRef<Node>, input: CommandInput) {
        match &input.cmd {
            Command::UI(UICommand { label, action }) if label.as_str() == "radar" => {
                let ui = base.cast::<Control>().unwrap();
                match action {
                    UIAction::Hide => ui.set_visible(false),
                    UIAction::Show => ui.set_visible(true),
                }
                base.emit_signal(
                    VMSignal::OnCmdResult,
                    &input.into_result(Ok("ok".to_owned())).as_var(),
                );
            }
            _ => return,
        }
    }

    #[method]
    fn _process(&self, #[base] base: &Node, _delta: f64) -> Option<()> {
        let player = unsafe { base.get_node(Player::path())?.assume_safe() }.cast::<Spatial>()?;
        for entry in self.detected.borrow().iter() {
            render_marker(base, player, entry);
        }
        Some(())
    }

    #[method]
    fn on_detected(&self, #[base] base: &Node, area: Ref<Area>) -> Option<()> {
        let enemy = unsafe { area.assume_safe().get_parent()?.assume_safe() };
        if !enemy.is_in_group(groups::ENEMY) {
            return None;
        }
        let id = enemy.name();
        self.detected.borrow_mut().insert(id.clone(), area);
        let enemy_chip = unsafe {
            base.get_node("D4")?
                .assume_safe()
                .duplicate(0)?
                .assume_safe()
        };
        enemy_chip.set_name(id);
        enemy_chip.set("visible", true);
        base.add_child(enemy_chip, false);
        Some(())
    }

    #[method]
    fn on_lost(&self, #[base] base: &Node, area: Ref<Area>) -> Option<()> {
        let id = unsafe { area.assume_safe().get_parent()?.assume_safe() }.name();
        self.detected.borrow_mut().remove(&id);
        base.remove_child(base.get_node(id)?);
        Some(())
    }
}

fn render_marker(
    base: &Node,
    player: TRef<Spatial>,
    (id, area): (&GodotString, &Ref<Area>),
) -> Option<()> {
    let node = base.get_node_or_null(id.to_owned())?;

    if let Some(ret) = unsafe { node.assume_safe() }.cast::<TextureRect>() {
        let vec = (unsafe { area.assume_safe() }.global_transform().origin
            - player.global_transform().origin)
            / 5.; // scale
        let rel = Vector2::new(vec.z + 50., -vec.x + 50.);

        ret.set_position(rel, false);
    }
    Some(())
}
