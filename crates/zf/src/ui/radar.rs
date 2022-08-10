use std::{cell::RefCell, collections::HashMap};

use gdnative::{
    api::{Area, TextureRect},
    prelude::*,
};

use crate::{
    path::HasPath,
    units::player::Player,
    vm::{register_vm_signal, Command, CommandInput, VMConnecter, VMSignal},
};

#[derive(NativeClass, Default)]
#[inherit(Node)]
#[register_with(register_vm_signal)]
pub struct Radar {
    detected: RefCell<HashMap<GodotString, Ref<Area>>>,
}

#[methods]
impl Radar {
    fn new(_owner: &Node) -> Self {
        Default::default()
    }

    #[export]
    fn _ready(&self, owner: TRef<Node>) -> Option<()> {
        owner.connect_vm_signal(VMSignal::OnCmdParsed.into());

        let player_radar = unsafe {
            owner
                .get_node(&format!("{}/{}", Player::path(), "RadarArea"))?
                .assume_safe()
        }
        .cast::<Area>()?;

        player_radar
            .connect(
                "area_entered",
                owner,
                "on_detected",
                VariantArray::new_shared(),
                0,
            )
            .expect("failed to connect area_entered");

        player_radar
            .connect(
                "area_exited",
                owner,
                "on_lost",
                VariantArray::new_shared(),
                0,
            )
            .expect("failed to connect area_exited");

        Some(())
    }

    #[export]
    fn on_cmd_parsed(&self, owner: &Node, input: CommandInput) {
        if !matches!(input.cmd, Command::Radar(_)) {
            return;
        }
        let msg = Ok(format!("{:?}", &self.detected));
        let res = input.into_result(msg);
        owner.emit_signal(VMSignal::OnCmdResult, &res.as_var());
    }

    #[export]
    fn _process(&self, owner: &Node, _delta: f64) -> Option<()> {
        let player = unsafe { owner.get_node(Player::path())?.assume_safe() }.cast::<Spatial>()?;
        for entry in self.detected.borrow().iter() {
            render_marker(owner, player, entry);
        }
        Some(())
    }

    #[export]
    fn on_detected(&self, owner: &Node, area: Ref<Area>) -> Option<()> {
        let id = unsafe { area.assume_safe().get_parent()?.assume_safe() }.name();
        self.detected.borrow_mut().insert(id.clone(), area);
        let enemy = unsafe {
            owner
                .get_node("pawn")?
                .assume_safe()
                .duplicate(0)?
                .assume_safe()
        };
        enemy.set_name(id);
        enemy.set("visible", true);
        owner.add_child(enemy, false);
        Some(())
    }

    #[export]
    fn on_lost(&self, owner: &Node, area: Ref<Area>) -> Option<()> {
        let id = unsafe { area.assume_safe().get_parent()?.assume_safe() }.name();
        self.detected.borrow_mut().remove(&id);
        owner.remove_child(owner.get_node(id)?);
        Some(())
    }
}

fn render_marker(
    owner: &Node,
    player: TRef<Spatial>,
    (id, area): (&GodotString, &Ref<Area>),
) -> Option<()> {
    let node = owner.get_node(id.to_owned())?;

    if let Some(ret) = unsafe { node.assume_safe() }.cast::<TextureRect>() {
        let vec = (unsafe { area.assume_safe() }.global_transform().origin
            - player.global_transform().origin)
            / 5.; // scale
        let rel = Vector2::new(vec.z + 50., -vec.x + 50.);

        ret.set_position(rel, false);
    }
    Some(())
}
