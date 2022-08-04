use gdnative::prelude::*;

use crate::{common::find_ref, managers::VMManager, vm::Command};

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

impl CommandResult {
    pub fn as_var(&self) -> [Variant; 1] {
        [Variant::new(self)]
    }
}

#[derive(Debug, PartialEq)]
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

impl Into<GodotString> for VMSignal {
    fn into(self) -> GodotString {
        self.as_str().into()
    }
}

impl Into<GodotString> for &VMSignal {
    fn into(self) -> GodotString {
        self.as_str().into()
    }
}

pub trait VMConnecter {
    fn connect_vm_signal(self, signal: VMSignal) -> Option<()>;
}

impl<'a> VMConnecter for TRef<'a, Node> {
    fn connect_vm_signal(self, signal: VMSignal) -> Option<()> {
        let vm_manager = find_ref::<VMManager, Node>(self)?;
        vm_manager
            .connect(
                &signal, // fmt
                self,
                &signal,
                VariantArray::new_shared(),
                0,
            )
            .expect(&format!("failed to connect vm {}", signal.as_str()));

        if VMSignal::OnCmdParsed == signal {
            self.connect(
                VMSignal::OnCmdResult,
                vm_manager,
                VMSignal::OnCmdResult,
                VariantArray::new_shared(),
                0,
            )
            .expect(&format!("failed to connect vm back {}", signal.as_str()));
        };
        Some(())
    }
}

pub fn register_vm_signal<T: NativeClass>(builder: &ClassBuilder<T>) {
    builder.signal(VMSignal::OnCmdResult.as_str()).done();
}
