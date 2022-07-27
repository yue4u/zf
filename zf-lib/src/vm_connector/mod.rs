mod vm_host;

use gdnative::prelude::*;
pub use vm_host::VMHost;

use crate::{
    common::{find_ref, HasPath},
    vm::Command,
};

#[derive(Debug, FromVariant, ToVariant)]
pub struct CommandInput {
    pub cmd: Command,
    pub index: u32,
}

pub type CommandResult = Result<String, String>;

pub fn connect_on_cmd_parsed(target: TRef<Node>) -> Option<()> {
    find_ref::<VMHost, Node>(target)?
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
    find_ref::<VMHost, Node>(target)?
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

pub fn send_result(target: &Node, result: CommandResult) -> Option<()> {
    unsafe { target.get_node_as_instance::<VMHost>(VMHost::path())? }
        .map(|host, _| {
            (*host).receive_command_result(result);
        })
        .ok()
}
