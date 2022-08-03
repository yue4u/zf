use gdnative::prelude::*;

use crate::{common::find_ref, managers::VMManager, path::HasPath, vm::Command};

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub struct CommandInput {
    pub cmd: Command,
    pub id: u32,
}

impl CommandInput {
    pub fn into_result(self, result: Result<String, String>) -> CommandResult {
        CommandResult {
            id: self.id,
            result,
        }
    }
}

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub struct CommandResult {
    pub id: u32,
    pub result: Result<String, String>,
}

pub trait CommandExecutor {
    fn send_vm_result(&self, result: CommandResult) -> Option<()>;
}

impl<T> CommandExecutor for T
where
    T: NodeResolveExt<&'static str>,
{
    fn send_vm_result(&self, result: CommandResult) -> Option<()> {
        godot_print!("before send result");
        let vm = unsafe { self.get_node_as_instance::<VMManager>(VMManager::path())? };
        let r = vm.map(|host, _| {
            (*host).receive_command_result(result);
        });
        godot_print!("{:?}", r);
        Some(())
    }
}

pub enum VMSignal {
    OnCmdEntered,
    OnCmdParsed,
    OnCmdResult,
}

impl VMSignal {
    pub fn as_str(&self) -> &'static str {
        match self {
            VMSignal::OnCmdEntered => "on_cmd_entered",
            VMSignal::OnCmdParsed => "on_cmd_parsed",
            VMSignal::OnCmdResult => "on_cmd_result",
        }
    }
}

pub trait VMConnecter {
    fn connect_vm_signal(self, signal: VMSignal) -> Option<()>;
}

impl<'a> VMConnecter for TRef<'a, Node> {
    fn connect_vm_signal(self, signal: VMSignal) -> Option<()> {
        let signal = signal.as_str();
        find_ref::<VMManager, Node>(self)?
            .connect(
                signal, // fmt
                self,
                signal,
                VariantArray::new_shared(),
                0,
            )
            .expect(&format!("failed to connect line edit {signal}"));
        Some(())
    }
}
