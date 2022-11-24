use gdnative::{
    api::{
        object::ConnectFlags,
        // Font,
        DynamicFont,
        GlobalConstants,
    },
    prelude::*,
};
use zf_term::{TerminalSize, ZFTerm, ZF};

use crate::{
    common::find_ref,
    managers::VMManager,
    refs,
    vm::{CommandResult, VMSignal},
};

#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_signals)]
pub struct Terminal {
    // seqno: usize,
    state: ZFTerm,
    buffer: String,
    font: Ref<DynamicFont>,
    cell_size: Vector2,
    // font: Ref<Font>,
}

const ENTER_SIGNAL: &'static str = "signal";

struct TerminalWriter {
    // base: Ref<Control>,
}

impl TerminalWriter {
    // fn prompt(&mut self) {
    //     use nu_ansi_term::Color::*;
    //     // let err = match self.state {
    //     //     ProcessState::Idle => "",
    //     //     ProcessState::Error => "[error]",
    //     //     _ => "",
    //     // };
    //     // self.write(&format!("\n{}{}", Red.paint(err), Cyan.paint("> ")));
    //     self.write(&format!("\n{}", Cyan.paint("> ")));
    // }
}

impl std::io::Write for TerminalWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // FIXME: we should send data "here"

        // FIXME: we should not converting back to string here again
        // let buffer = String::from_utf8(buf.to_vec()).unwrap();
        // match buffer.as_str() {
        //     // "clear" => {
        //     //     self.write(&"\n".repeat(20));
        //     //     self.prompt()
        //     // }
        //     lines => {
        //         godot_print!("send: {}", lines);
        //         unsafe { self.base.assume_safe() }.emit_signal(ENTER_SIGNAL, &[lines.to_variant()]);
        //         // self.state = ProcessState::Running;
        //         // self.buffer = "".to_string();
        //     }
        // }
        // // self.buffer = "".to_string();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[methods]
impl Terminal {
    fn new(base: TRef<Control>) -> Self {
        let font = ResourceLoader::godot_singleton()
            .load(
                refs::path::assets::JET_BRAINS_MONO_TRES,
                "DynamicFont",
                false,
            )
            .unwrap()
            .cast::<DynamicFont>()
            .unwrap();

        // let font = unsafe { base.get_font("", "").unwrap().assume_safe() }.claim();
        let writer = Box::new(TerminalWriter {});
        let cell_size = unsafe { font.assume_safe() }.get_string_size("W");

        Terminal {
            // seqno: 0,
            font,
            buffer: String::new(),
            cell_size,
            state: ZFTerm::new(
                writer,
                TerminalSize {
                    rows: 40,
                    ..Default::default()
                },
            ),
        }
    }

    pub(crate) fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal(ENTER_SIGNAL).done();
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Control>) -> Option<()> {
        base.grab_focus();

        base.connect(
            "gui_input",
            base,
            "on_gui_input",
            VariantArray::new_shared(),
            0,
        )
        .expect("failed to connect on_gui_input");

        let as_node = unsafe { base.get_node_as::<Node>(".")? };
        let vm_manager = find_ref::<VMManager, Node>(as_node)?;

        base.connect(
            ENTER_SIGNAL,
            vm_manager,
            VMSignal::OnCmdEntered,
            VariantArray::new_shared(),
            ConnectFlags::DEFERRED.into(),
        )
        .expect("failed to connect vm {}");

        vm_manager
            .connect(
                VMSignal::OnCmdResult,
                base,
                VMSignal::OnCmdResult,
                VariantArray::new_shared(),
                ConnectFlags::DEFERRED.into(),
            )
            .expect("failed to connect vm {}");

        self.write(ZF);
        // self.prompt();
        // // TODO: size_changed
        Some(())
    }

    fn write(&mut self, data: &str) {
        self.state.term.advance_bytes(data);
    }

    #[method]
    fn on_gui_input(&mut self, #[base] base: &Control, event: Ref<InputEvent>) -> Option<()> {
        let event = unsafe { event.assume_safe() }.cast::<InputEventKey>()?;

        // skip if not pressed
        if !event.is_pressed() {
            return Some(());
        }

        // if self.state == ProcessState::Running {
        //     return None;
        // }

        match event.scancode() {
            GlobalConstants::KEY_ENTER => {
                match self.buffer.as_str() {
                    "clear" => {
                        self.write("\033[2J");
                        self.prompt()
                    }
                    lines => {
                        // godot_dbg!("lines: {}", lines);
                        // base.emit_signal(ENTER_SIGNAL, &[self.buffer.to_variant()]);
                        // self.state = ProcessState::Running;
                        // let buffer: String = self.buffer.drain(..).collect();
                        godot_dbg!(lines);
                        base.emit_signal(ENTER_SIGNAL, &[lines.to_variant()]);

                        // self.state
                        //     .term
                        //     .send_paste(&lines)
                        //     .expect("failed to send_paste");
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
            GlobalConstants::KEY_BACKSPACE => {
                if !self.buffer.is_empty() {
                    self.buffer = self.buffer[..self.buffer.len() - 1].to_string();
                    self.write("\x08 \x08");
                }
            }
            _ => {
                let ch = event.unicode() as u8 as char;
                if ch != '\0' && ch != '\r' {
                    godot_dbg!(&ch);
                    // self.state.term.send_paste(&String::from(ch));
                    self.buffer.push(ch);
                    self.write(&ch.to_string());
                    // self.state.term.wr
                }
            }
        }
        base.update();
        Some(())
    }

    #[method]
    fn _draw(&mut self, #[base] base: &Control) {
        let rect = base.get_rect();
        base.draw_rect(rect, Color::from_rgba(0., 0., 0., 0.5), true, -1., false);
        self.state
            .term
            .screen_mut()
            .for_each_phys_line_mut(|y, line| {
                let mut x = 0;
                for cell in line.cells_mut() {
                    base.draw_string(
                        &self.font,
                        Vector2 {
                            x: rect.position.x + (x + 1) as f32 * self.cell_size.x,
                            y: rect.position.y + (y + 1) as f32 * self.cell_size.y,
                        },
                        cell.str(),
                        Color::from_rgba(1., 1., 1., 1.),
                        -1,
                    );
                    x += cell.width();
                }
            });
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
    fn on_cmd_result(&mut self, #[base] base: &Control, result: CommandResult) -> Option<()> {
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
        base.update();
        Some(())
    }
}
