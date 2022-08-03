use gdnative::prelude::*;

use crate::{common::find_ref, managers::VMManger, path::HasPath, vm::Command};

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub struct CommandInput {
    pub cmd: Command,
    pub id: u32,
}

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub struct CommandResult {
    pub id: u32,
    pub result: Result<String, String>,
}

pub fn connect_on_cmd_parsed(target: TRef<Node>) -> Option<()> {
    find_ref::<VMManger, Node>(target)?
        .connect(
            "on_cmd_parsed",
            target,
            "on_cmd_parsed",
            VariantArray::new_shared(),
            0,
        )
        .expect("failed to connect line edit on_cmd_parsed");
    Some(())
}

pub fn connect_on_cmd_entered(target: TRef<Node>) -> Option<()> {
    find_ref::<VMManger, Node>(target)?
        .connect(
            "on_cmd_entered",
            target,
            "on_cmd_entered",
            VariantArray::new_shared(),
            0,
        )
        .expect("failed to connect line edit");
    Some(())
}

pub fn connect_on_cmd_result(target: TRef<Node>) -> Option<()> {
    find_ref::<VMManger, Node>(target)?
        .connect(
            "on_cmd_result",
            target,
            "on_cmd_result",
            VariantArray::new_shared(),
            0,
        )
        .expect("failed to connect line edit");
    Some(())
}

pub trait CommandExecutor {
    fn send_result(&self, result: CommandResult) -> Option<()>;
}

impl<T> CommandExecutor for T
where
    T: NodeResolveExt<&'static str>,
{
    fn send_result(&self, result: CommandResult) -> Option<()> {
        godot_print!("before send result");
        let vm = unsafe { self.get_node_as_instance::<VMManger>(VMManger::path())? };
        let r = vm.map(|host, _| {
            (*host).receive_command_result(result);
        });
        godot_print!("{:?}", r);
        Some(())
    }
}
