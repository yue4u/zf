use gdnative::{
    api::{object::ConnectFlags, GlobalConstants},
    prelude::*,
};

use crate::{
    common::find_ref,
    managers::VMManager,
    vm::{CommandResult, ProcessState, VMSignal},
};

#[derive(NativeClass, Debug, Default)]
#[inherit(Control)]
#[register_with(Self::register_signals)]
pub struct TerminalWrap {
    state: ProcessState,
    buffer: String,
    term: Option<Ref<Node>>,
}

const ENTER_SIGNAL: &'static str = "signal";
const ZF: &'static str = r#"
___          _        _     _______
| _ \_ _ ___ (_)___ __| |_  |_  / __|
|  _/ '_/ _ \| / -_) _|  _|  / /| _|
|_| |_| \___// \___\__|\__| /___|_|
           |__/

Weclome to zf console!
"#;

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
            term.call_deferred("grab_focus", &[]);
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

        self.write(ZF);
        self.prompt();
        // TODO: size_changed
        Some(())
    }

    fn write(&self, data: &str) {
        unsafe {
            self.term
                .expect("term should be ready")
                .assume_safe()
                .call("write", &[data.replace("\n", "\r\n").to_variant()]);
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

        if self.state == ProcessState::Running {
            return None;
        }

        match event.scancode() {
            GlobalConstants::KEY_ENTER => {
                match self.buffer.as_str() {
                    "clear" => {
                        self.write(&"\n".repeat(20));
                        self.prompt()
                    }
                    lines => {
                        godot_dbg!("lines: {}", lines);
                        let result = zf_runtime::Runtime::eval(lines);
                        self.write("\n");
                        match result {
                            Ok(result) => {
                                self.write(&result);
                            }
                            Err(err) => {
                                self.write(&err.to_string());
                            }
                        }
                        self.prompt()
                        // base.emit_signal(ENTER_SIGNAL, &[self.buffer.to_variant()]);
                        // self.state = ProcessState::Running;
                        // self.buffer = "".to_string();
                    }
                }
                self.buffer = "".to_string();
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
                self.write(&char.to_string());
            }
        }
        Some(())
    }

    fn prompt(&self) {
        use nu_ansi_term::Color::*;
        let err = match self.state {
            ProcessState::Idle => "",
            ProcessState::Error => "[error]",
            _ => "",
        };
        self.write(&format!("\n{}{}", Red.paint(err), Cyan.paint("> ")));
    }

    #[method]
    fn on_cmd_result(&mut self, result: CommandResult) -> Option<()> {
        let result = match result.result {
            Ok(result) => {
                self.state = ProcessState::Idle;
                result
            }
            Err(_) => {
                self.state = ProcessState::Error;
                format!("{:?}", result)
            }
        };
        godot_dbg!(&result);
        self.write("\n");
        self.write(&result);
        self.prompt();
        Some(())
    }
}
