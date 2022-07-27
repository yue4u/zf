mod vm_host;

use gdnative::prelude::*;
pub use vm_host::VMHost;

use crate::{common::find_ref, vm::Command};

#[derive(Debug, FromVariant, ToVariant)]
pub struct CommandInput {
    pub cmd: Command,
    pub index: u32,
}

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
