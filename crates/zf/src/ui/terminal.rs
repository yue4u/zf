use gdnative::{
    api::{
        object::ConnectFlags,
        // Font,
        DynamicFont,
        GlobalConstants,
    },
    prelude::*,
};
use zf_runtime::strip_ansi;
use zf_term::{TerminalSize, ZFTerm, ZF};

use crate::{
    common::{current_scene, find_ref},
    entities::GLOBAL_GAME_STATE,
    managers::VMManager,
    refs::{self, path::SceneName, HasPath},
    vm::{CommandResult, VMSignal},
};

#[derive(FromVariant, ToVariant, Clone, Debug, PartialEq)]
pub enum ProcessState {
    Idle,
    Done,
    Error,
    Running,
}

impl Default for ProcessState {
    fn default() -> Self {
        ProcessState::Idle
    }
}

#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_signals)]
pub struct Terminal {
    // seqno: usize,
    state: ZFTerm,
    process_state: ProcessState,
    buffer: String,
    font: Ref<DynamicFont>,
    cell_size: Vector2,
    // font: Ref<Font>,
}

impl HasPath for Terminal {
    fn path() -> &'static str {
        refs::path::base::TERMINAL
    }
}

const TERM_PADDING: f32 = 10.;
const ENTER_SIGNAL: &'static str = "signal";
// const GAME_INFO_SIGNAL: &'static str = "game_info";

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
    fn new(_base: TRef<Control>) -> Self {
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
            process_state: ProcessState::Idle,
            buffer: String::new(),
            cell_size,
            state: ZFTerm::new(writer, TerminalSize::default()),
        }
    }

    pub fn get_size(&self) -> TerminalSize {
        self.state.term.get_size()
    }

    #[method]
    fn resize(&mut self, #[base] base: TRef<Control>) {
        let term_size = calc_terminal_size(base, self.cell_size);
        self.state.term.resize(term_size);
    }

    pub(crate) fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal(ENTER_SIGNAL).done();
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Control>) -> Option<()> {
        self.resize(base);

        base.grab_focus();

        base.connect(
            "gui_input",
            base,
            "on_gui_input",
            VariantArray::new_shared(),
            0,
        )
        .expect("failed to connect on_gui_input");

        base.connect("resized", base, "resize", VariantArray::new_shared(), 0)
            .expect("failed to connect resize");

        let as_node = unsafe { base.get_node_as::<Node>(".")? };
        let vm_manager = find_ref::<VMManager, Node>(as_node)?;

        base.connect(
            ENTER_SIGNAL,
            vm_manager,
            VMSignal::OnCmdEntered,
            VariantArray::new_shared(),
            ConnectFlags::DEFERRED.into(),
        )
        .expect("failed to connect vm");

        vm_manager
            .connect(
                VMSignal::OnCmdResult,
                base,
                VMSignal::OnCmdResult,
                VariantArray::new_shared(),
                ConnectFlags::DEFERRED.into(),
            )
            .expect("failed to connect vm");

        self.write(ZF);
        self.write_scene_message(base);
        self.prompt();

        Some(())
    }

    fn write(&mut self, data: &str) {
        self.state.term.advance_bytes(data);
    }

    #[method]
    fn write_scene_message(&mut self, #[base] base: TRef<Control>) -> Option<()> {
        use nu_ansi_term::Color::*;

        let as_node = unsafe { base.get_node_as::<Node>(".")? };
        let code = Rgb(255, 194, 60).bold();

        let text = match current_scene(&as_node) {
            SceneName::Sandbox => None,
            SceneName::StartMenu => Some(format!(
                "Type {} to continue or {} for help.",
                code.paint(
                    GLOBAL_GAME_STATE
                        .lock()
                        .map(|state| {
                            if state.tutorial_completed {
                                "game start"
                            } else {
                                "game tutorial"
                            }
                        })
                        .unwrap_or_else(|_| "game tutorial")
                ),
                code.paint("help")
            )),
            SceneName::TutorialMovement => Some(format!(
                "type {} to explore engine command",
                code.paint("engine --help")
            )),
            _ => None,
        }?;
        let line = format!(
            "\n{}\n",
            DarkGray.paint(
                "=".repeat(
                    strip_ansi(&text)
                        .lines()
                        .map(|l| l.len())
                        .max()
                        .unwrap_or_default()
                )
            )
        );
        self.write(&line);
        self.write(&text);
        self.write(&line);
        None
    }

    #[method]
    fn on_gui_input(&mut self, #[base] base: &Control, event: Ref<InputEvent>) -> Option<()> {
        let event = unsafe { event.assume_safe() }.cast::<InputEventKey>()?;

        // skip if not pressed
        if !event.is_pressed() {
            return Some(());
        }

        if self.process_state == ProcessState::Running {
            return None;
        }

        match event.scancode() {
            GlobalConstants::KEY_ENTER => {
                match self.buffer.as_str() {
                    "clear" => {
                        self.write("\x1b[2J");
                        self.prompt()
                    }
                    lines => {
                        // godot_dbg!("lines: {}", lines);
                        // base.emit_signal(ENTER_SIGNAL, &[self.buffer.to_variant()]);
                        // self.state = ProcessState::Running;
                        // let buffer: String = self.buffer.drain(..).collect();
                        // godot_dbg!(lines);
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
                    self.buffer.push(ch);
                    self.write(&ch.to_string());
                }
            }
        }
        base.update();
        Some(())
    }

    #[method]
    fn _draw(&mut self, #[base] base: &Control) {
        let rect = base.get_rect();
        // godot_dbg!(rect);
        let color_palette = &self.state.term.get_config().color_palette();
        let anchor = Vector2 { x: 0., y: 0. };
        base.draw_rect(
            Rect2 {
                position: anchor,
                size: rect.size,
            },
            Color::from_rgba(0., 0., 0., 0.5),
            true,
            -1.,
            false,
        );

        let screen = self.state.term.screen_mut();

        // TODO: this is very wrong and better to use index api
        let mut lines = Vec::new();

        screen.for_each_phys_line_mut(|_y, line| {
            lines.push(line.clone());
        });

        let lines_len = lines.len();

        let draw_pos = |x: f32, y: f32| Vector2 {
            x: TERM_PADDING + x * self.cell_size.x,
            // position uses bottom-left so 2x here
            y: 2. * TERM_PADDING + y * self.cell_size.y,
        };

        lines
            .iter_mut()
            .skip(lines_len.saturating_sub(screen.physical_rows))
            .enumerate()
            .for_each(|(y, line)| {
                let mut x = 0;
                for cell in line.cells_mut() {
                    let fg = zf_term::Color::resolve_cell_fg_color(cell, color_palette);
                    let bg = zf_term::Color::resolve_cell_bg_color(cell, color_palette);

                    // base.draw_rect(
                    //     Rect2 {
                    //         position: Vector2 {
                    //             x: (x as f32) * self.cell_size.x,
                    //             y: (y as f32) * self.cell_size.y,
                    //         },
                    //         size: Vector2 {
                    //             x: cell.width() as f32 * self.cell_size.x,
                    //             y: self.cell_size.y,
                    //         },
                    //     },
                    //     Color::from_rgba(bg.0, bg.1, bg.2, bg.3),
                    //     true,
                    //     -1.,
                    //     false,
                    // );

                    // HACK: using draw_rect with draw_string has z index issues
                    base.draw_string(
                        &self.font,
                        draw_pos(x as f32, y as f32),
                        "█".repeat(cell.width()),
                        Color::from_rgba(bg.0, bg.1, bg.2, bg.3),
                        -1,
                    );

                    base.draw_string(
                        &self.font,
                        draw_pos(x as f32, y as f32),
                        cell.str(),
                        Color::from_rgba(fg.0, fg.1, fg.2, fg.3),
                        -1,
                    );
                    x += cell.width();
                }
            });

        // cursor
        let cursor_pos = self.state.term.cursor_pos();
        base.draw_char(
            &self.font,
            draw_pos(cursor_pos.x as f32, cursor_pos.y as f32),
            "_",
            "",
            Color::from_rgba(1., 1., 1., 1.),
        );
    }

    fn prompt(&mut self) {
        use nu_ansi_term::Color::*;
        let before = match self.process_state {
            ProcessState::Error => Black.on(LightRed).paint(" "),
            _ /*     fmt     */ => Black.on(LightCyan).paint(" "),
        };
        self.write(&format!("\n{} > ", before));
    }

    #[method]
    fn on_cmd_result(&mut self, #[base] base: &Control, result: CommandResult) -> Option<()> {
        let result = match result.result {
            Ok(result) => {
                self.process_state = ProcessState::Idle;
                result
            }
            Err(result) => {
                self.process_state = ProcessState::Error;
                result
            }
        };

        self.write("\n");
        self.write(&result);
        self.prompt();
        base.update();
        Some(())
    }
}

fn calc_terminal_size(base: TRef<Control>, cell_size: Vector2) -> TerminalSize {
    let rect = base.get_rect();
    let rows = ((rect.size.y - TERM_PADDING * 2.) / cell_size.y).floor() as usize;
    let cols = ((rect.size.x - TERM_PADDING * 2.) / cell_size.x).floor() as usize;

    TerminalSize {
        rows,
        cols,
        ..Default::default()
    }
}
