use gdnative::{api::object::ConnectFlags, prelude::*};
use zf_ffi::CommandArgs;

use crate::{common::find_ref, managers::VMManager};

#[derive(Debug, FromVariant, ToVariant, Clone)]
pub struct CommandInput {
    pub cmd: CommandArgs,
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

pub struct VMSignalConnectOptions {
    pub signal: VMSignal,
    pub bidirectional: bool,
}

impl VMSignalConnectOptions {
    pub fn bidirectional(mut self, bidirectional: bool) -> Self {
        self.bidirectional = bidirectional;
        self
    }
}

impl Into<VMSignalConnectOptions> for VMSignal {
    fn into(self) -> VMSignalConnectOptions {
        // defualt bidirectional is true for OnCmdParsed
        let bidirectional = self == VMSignal::OnCmdParsed;

        VMSignalConnectOptions {
            signal: self,
            bidirectional,
        }
    }
}

impl VMSignal {
    pub fn to_options(self) -> VMSignalConnectOptions {
        self.into()
    }

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
    fn connect_vm_signal(self, signal: VMSignalConnectOptions) -> Option<()>;
}

impl<'a> VMConnecter for TRef<'a, Node> {
    fn connect_vm_signal(self, config: VMSignalConnectOptions) -> Option<()> {
        let vm_manager = find_ref::<VMManager, Node>(self)?;
        vm_manager
            .connect(
                &config.signal,
                self,
                &config.signal,
                VariantArray::new_shared(),
                ConnectFlags::DEFERRED.into(),
            )
            .expect(&format!("failed to connect vm {}", config.signal.as_str()));

        if config.bidirectional {
            self.connect(
                VMSignal::OnCmdResult,
                vm_manager,
                VMSignal::OnCmdResult,
                VariantArray::new_shared(),
                ConnectFlags::DEFERRED.into(),
            )
            .expect(&format!(
                "failed to connect vm back {}",
                config.signal.as_str()
            ));
        };
        Some(())
    }
}

pub fn register_vm_signal<T: NativeClass>(builder: &ClassBuilder<T>) {
    builder.signal(VMSignal::OnCmdResult.as_str()).done();
}
