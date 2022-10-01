use gdnative::{
    api::{object::ConnectFlags, GlobalConstants},
    prelude::*,
};

use crate::{
    common::find_ref,
    managers::VMManager,
    vm::{CommandResult, VMSignal},
};

#[derive(NativeClass, Debug, Default)]
#[inherit(Control)]
#[register_with(Self::register_signals)]
pub struct TerminalWrap {
    buffer: String,
    term: Option<Ref<Node>>,
}

const ENTER_SIGNAL: &'static str = "signal";

#[methods]
impl TerminalWrap {
    fn new(_base: &Control) -> Self {
        TerminalWrap::default()
    }

    pub(crate) fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal(ENTER_SIGNAL).done();
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Control>) -> Option<()> {
        let term = unsafe { base.get_node("./Terminal")?.assume_safe() };

        term.connect(
            "key_pressed",
            base,
            "on_key_pressed",
            VariantArray::new_shared(),
            0,
        )
        .expect("failed to connect on_key_pressed");

        unsafe {
            term.call("grab_focus", &[]);
        }

        let as_node = unsafe { base.get_node_as::<Node>(".")? };
        let vm_manager = find_ref::<VMManager, Node>(as_node)?;

        base.connect(
            ENTER_SIGNAL,
            vm_manager,
            VMSignal::OnCmdEntered,
            VariantArray::new_shared(),
            ConnectFlags::DEFERRED.into(),
        )
        .expect(&format!("failed to connect vm {}", ""));

        vm_manager
            .connect(
                VMSignal::OnCmdResult,
                base,
                VMSignal::OnCmdResult,
                VariantArray::new_shared(),
                ConnectFlags::DEFERRED.into(),
            )
            .expect(&format!("failed to connect vm {}", ""));

        self.term = Some(term.claim());

        godot_print!("terminal wrap ready");
        self.write("terminal wrap ready");
        // TODO: size_changed
        Some(())
    }

    fn write(&self, data: impl ToVariant) {
        unsafe {
            self.term
                .expect("term should be ready")
                .assume_safe()
                .call("write", &[data.to_variant()]);
        }
    }

    #[method]
    fn on_key_pressed(
        &mut self,
        #[base] base: &Control,
        _data: Variant,
        event: Ref<InputEvent>,
    ) -> Option<()> {
        let event = unsafe { event.assume_safe() }.cast::<InputEventKey>()?;

        match event.scancode() {
            GlobalConstants::KEY_ENTER => {
                base.emit_signal(ENTER_SIGNAL, &[self.buffer.to_variant()]);
                self.buffer = "".to_string();
                self.write("\r\n");
            }
            GlobalConstants::KEY_BACKSPACE => {
                if !self.buffer.is_empty() {
                    self.buffer = self.buffer[..self.buffer.len() - 1].to_string();
                    self.write("\x08 \x08");
                }
            }
            _ => {
                let char = event.unicode() as u8 as char;
                self.buffer.push(char);
                self.write(char.to_string());
            }
        }
        Some(())
    }

    #[method]
    fn on_cmd_result(&self, result: CommandResult) -> Option<()> {
        let result = match result.result {
            Ok(result) => result,
            Err(_) => format!("{:?}", result),
        };
        self.write("\r\n");
        self.write(result);
        Some(())
    }
}
