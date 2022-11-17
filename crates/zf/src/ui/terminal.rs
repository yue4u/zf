use gdnative::{
    api::{object::ConnectFlags, GlobalConstants, OS},
    prelude::*,
};
use zf_term::ZFTermState;

use crate::{
    common::find_ref,
    managers::VMManager,
    vm::{CommandResult, ProcessState, VMSignal},
};

#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_signals)]
pub struct Terminal {
    state: ZFTermState,
    buffer: String,
    // term: Option<Ref<Node>>,
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

struct TerminalWriter {
    // buffer: String,
    base: Ref<Control>,
}

impl std::io::Write for TerminalWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // dbg!(buf);
        // Ok(buf.len())

        // match self.buffer.as_str() {
        // "clear" => {
        //     self.write(&"\n".repeat(20));
        //     self.prompt()
        // }
        // lines => {
        // godot_dbg!("lines: {}", lines);
        unsafe { self.base.assume_safe() }.emit_signal(
            ENTER_SIGNAL,
            // FIXME: String should not be used here
            &[String::from_utf8_lossy(buf).to_string().to_variant()],
        );
        // self.state = ProcessState::Running;
        // self.buffer = "".to_string();
        //     }
        // };
        // self.buffer = "".to_string();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[methods]
impl Terminal {
    fn new(base: TRef<Control>) -> Self {
        let writer = Box::new(TerminalWriter {
            // buffer: String::new(),
            base: base.claim(),
        });

        Terminal {
            buffer: String::new(),
            state: ZFTermState::new(writer),
        }
    }

    pub(crate) fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal(ENTER_SIGNAL).done();
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Control>) -> Option<()> {
        // let term = unsafe { base.get_node("./Terminal")?.assume_safe() };

        base.connect(
            "gui_input",
            base,
            "on_key_pressed",
            VariantArray::new_shared(),
            0,
        )
        .expect("failed to connect on_key_pressed");

        // unsafe {
        //     term.call_deferred("grab_focus", &[]);
        // }

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

        // self.term = Some(term.claim());

        self.write(ZF);
        // self.prompt();
        // // TODO: size_changed
        Some(())
    }

    fn write(&mut self, data: &str) {
        // unsafe {
        self.state.term.advance_bytes(data);
        // .expect("term should be ready")
        // .assume_safe()
        // .call("write", &[data.replace("\n", "\r\n").to_variant()]);
        // }
    }

    // fn _process(&mut self, #[base] base: TRef<Control>, delta: f64) -> Option<()> {

    #[method]
    fn draw(
        &mut self,
        // #[base] _base: TRef<Control>
    ) -> Option<()> {
        let mut buf = String::new();
        self.state
            .term
            .screen_mut()
            .for_each_phys_line_mut(|_, line| {
                for cell in line.cells_mut() {
                    if cell.str() != " " {
                        buf.push_str(cell.str());
                    }

                    // if !cell.str()
                    // base.draw_char(font, position, cell.text, next, modulate)
                }
                buf.push_str("\n");
            });
        godot_print!("{}", buf);
        // let env = base.cast::<WorldEnvironment>()?.environment()?;
        // let env = unsafe { env.assume_safe() };
        // let mut degrees = env.sky_rotation_degrees();
        // degrees.y -= delta as f32;
        // env.set_sky_rotation_degrees(degrees);
        Some(())
    }

    #[method]
    fn on_key_pressed(
        &mut self,
        #[base] _base: &Control,
        // _data: Variant,
        event: Ref<InputEvent>,
    ) -> Option<()> {
        // godot_dbg!("on_key_pressed", event);

        let event = unsafe { event.assume_safe() }.cast::<InputEventKey>()?;

        // if self.state == ProcessState::Running {
        //     return None;
        // }

        match event.scancode() {
            GlobalConstants::KEY_ENTER => {
                match self.buffer.as_str() {
                    // "clear" => {
                    //     self.write(&"\n".repeat(20));
                    //     self.prompt()
                    // }
                    lines => {
                        // godot_dbg!("lines: {}", lines);
                        // base.emit_signal(ENTER_SIGNAL, &[self.buffer.to_variant()]);
                        // self.state = ProcessState::Running;
                        // let buffer: String = self.buffer.drain(..).collect();
                        self.state
                            .term
                            .send_paste(&lines)
                            .expect("failed to send_paste");
                        // self.buffer = "".to_string();
                    }
                }
                self.buffer = "".to_string();
            }
            // GlobalConstants::KEY_V if event.control() => {
            //     let clipboard = OS::godot_singleton().clipboard().to_string();
            //     let clipboard_str = clipboard.as_str();
            //     self.buffer.push_str(clipboard_str);
            //     self.write(clipboard_str);
            // }
            // GlobalConstants::KEY_C if event.control() => {
            //     self.buffer.clear();
            //     self.prompt();
            // }
            // GlobalConstants::KEY_BACKSPACE => {
            //     if !self.buffer.is_empty() {
            //         self.buffer = self.buffer[..self.buffer.len() - 1].to_string();
            //         self.write("\x08 \x08");
            //     }
            // }
            _ => {
                let char = event.unicode() as u8 as char;
                // self.state.term.send_paste(&String::from(char));
                self.buffer.push(char);
                self.write(&char.to_string());
                // self.state.term.wr
            }
        }
        self.draw();
        Some(())
    }

    fn prompt(&mut self) {
        use nu_ansi_term::Color::*;
        // let err = match self.state {
        //     ProcessState::Idle => "",
        //     ProcessState::Error => "[error]",
        //     _ => "",
        // };
        // self.write(&format!("\n{}{}", Red.paint(err), Cyan.paint("> ")));
        self.write(&format!("\n{}", Cyan.paint("> ")));
    }

    #[method]
    fn on_cmd_result(&mut self, result: CommandResult) -> Option<()> {
        let result = match result.result {
            Ok(result) => {
                // self.state = ProcessState::Idle;
                result
            }
            Err(_) => {
                // self.state = ProcessState::Error;
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
