use gdnative::{
    api::{
        control::FocusMode,
        object::ConnectFlags,
        // Font,
        DynamicFont,
        GlobalConstants,
        ItemList,
        Particles2D,
    },
    prelude::*,
};
use zf_runtime::{cmds, strip_ansi};
use zf_term::{TerminalSize, ZFTerm, ZF};

use crate::{
    common::{current_scene, find_ref, PackedSceneRef, SceneLoader},
    entities::{GameState, GLOBAL_GAME_STATE},
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
    base: Ref<Control>,
    state: ZFTerm,
    process_state: ProcessState,
    buffer: String,
    font: Ref<DynamicFont>,
    // font: Ref<Font>,
    cell_size: Vector2,
    cmds: Vec<&'static str>,
    completion_item_list: Ref<ItemList>,
    typing_particles: PackedSceneRef,
}

impl HasPath for Terminal {
    fn path() -> &'static str {
        refs::path::auto_load::TERMINAL
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
        //         tracing::info!("send: {}", lines);
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
        let completion_item_list = ItemList::new().into_shared();
        Terminal {
            // seqno: 0,
            base: base.claim(),
            font,
            process_state: ProcessState::Idle,
            buffer: String::new(),
            cell_size,
            cmds: cmds(),
            state: ZFTerm::new(writer, TerminalSize::default()),
            completion_item_list,
            typing_particles: SceneLoader::load(refs::path::scenes::TYPING_PARTICLES).unwrap(),
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

        base.add_child(self.completion_item_list, false);

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
                VMSignal::OnGameState,
                base,
                VMSignal::OnGameState,
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
        self.write_scene_message(current_scene(&as_node));
        self.prompt();

        Some(())
    }

    fn write(&mut self, data: &str) {
        self.state.term.advance_bytes(data);
    }

    fn write_with_effect(&mut self, data: &str) {
        self.create_typing_particles();
        self.state.term.advance_bytes(data);
    }

    fn prompt(&mut self) {
        use nu_ansi_term::Color::*;
        let before = match self.process_state {
            ProcessState::Error => Black.on(LightRed).paint(" "),
            _ /*     fmt     */ => Black.on(LightCyan).paint(" "),
        };
        self.write(&format!("\n{} > ", before));
    }

    fn clear(&mut self) {
        self.write("\x1b[2J\x1b[H");
    }

    #[method]
    fn write_scene_message(&mut self, scene_name: SceneName) -> Option<()> {
        use nu_ansi_term::Color::*;

        let code = Rgb(255, 194, 60).bold();

        let text = match scene_name {
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

    fn create_typing_particles(&self) {
        let base = unsafe { self.base.assume_unique() };
        let cursor_pos = self.state.term.cursor_pos();
        let draw_pos = self.draw_pos(cursor_pos.x as f32, cursor_pos.y as f32);
        let typing_particles = SceneLoader::instance_as::<Particles2D>(&self.typing_particles)
            .unwrap()
            .into_shared();
        let typing_particles = unsafe { typing_particles.assume_safe() }.clone();
        typing_particles.set_emitting(true);
        typing_particles.set_position(draw_pos);
        base.add_child(typing_particles, false);

        for child in base.get_children().into_iter() {
            if let Some(particles_ref) = child.to_object::<Particles2D>() {
                let particles = unsafe { particles_ref.assume_safe() };
                if !particles.is_emitting() {
                    particles.queue_free();
                }
            }
        }
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

        let cl = unsafe { self.completion_item_list.assume_safe() };
        let mut cl_visible = false;
        let mut has_delta = true;

        let calc_selected = || {
            let selected_items = cl.get_selected_items();
            if selected_items.is_empty() {
                return None;
            }
            let item_count = cl.get_item_count();
            if item_count <= 1 {
                return None;
            }
            Some((item_count, selected_items.get(0) as i64))
        };

        match event.scancode() {
            GlobalConstants::KEY_ENTER => {
                match self.buffer.as_str() {
                    lines => {
                        // tracing::debug!("{:?}","lines: {}", lines);
                        // base.emit_signal(ENTER_SIGNAL, &[self.buffer.to_variant()]);
                        // self.state = ProcessState::Running;
                        // let buffer: String = self.buffer.drain(..).collect();
                        // tracing::debug!("{:?}",lines);
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
            GlobalConstants::KEY_TAB if cl.is_visible() => {
                // we are using single mode so only one
                let selected_items = cl.get_selected_items();
                if selected_items.is_empty() {
                    return None;
                }
                let selected = selected_items.get(0);
                let text = cl.get_item_text(selected as i64).to_string();
                let remain = text.strip_prefix(&self.buffer)?;
                self.write_with_effect(remain);
                self.buffer = text;
            }
            GlobalConstants::KEY_UP if cl.is_visible() => {
                has_delta = false;
                let (item_count, selected) = calc_selected()?;
                cl.select((item_count + selected - 1) % item_count, true);
            }
            GlobalConstants::KEY_DOWN if cl.is_visible() => {
                has_delta = false;
                let (item_count, selected) = calc_selected()?;
                cl.select((selected + 1) % item_count, true);
            }
            GlobalConstants::KEY_BACKSPACE => {
                if !self.buffer.is_empty() {
                    self.buffer = self.buffer[..self.buffer.len() - 1].to_string();
                    self.write_with_effect("\x08 \x08");
                }
            }
            _ => {
                let ch = event.unicode() as u8 as char;
                if ch != '\0' && ch != '\r' {
                    self.buffer.push(ch);
                    self.write_with_effect(&ch.to_string());
                }
            }
        }

        if !self.buffer.is_empty() {
            let matched: Vec<&&str> = self
                .cmds
                .iter()
                .filter(|cmd| cmd.starts_with(&self.buffer) && cmd.len() != self.buffer.len())
                .take(5)
                .collect();
            let matched_len = matched.len();
            if !matched.is_empty() {
                cl_visible = true;

                if has_delta {
                    cl.set_focus_mode(FocusMode::NONE.into());

                    cl.set_size(
                        Vector2 {
                            x: 200.,
                            y: matched_len as f32 * self.cell_size.y,
                        },
                        false,
                    );

                    cl.clear();
                    for item in matched.into_iter() {
                        cl.add_item(item, GodotObject::null(), true);
                    }

                    let cursor_pos = self.state.term.cursor_pos();
                    cl.set_position(
                        self.draw_pos(
                            (cursor_pos.x) as f32 + 0.5, // with extra adjust
                            // subtract display len + 1
                            (cursor_pos.y - matched_len as i64) as f32 - 0.5, // with extra adjust
                        ),
                        false,
                    );

                    // init selected idx
                    if cl.get_selected_items().len() < 1 {
                        cl.select(0, true);
                    }
                }
            }
        }

        cl.set_visible(cl_visible);
        base.update();
        Some(())
    }

    fn draw_pos(&self, x: f32, y: f32) -> Vector2 {
        Vector2 {
            x: TERM_PADDING + x * self.cell_size.x,
            // position uses bottom-left so 2x here
            y: 2. * TERM_PADDING + y * self.cell_size.y,
        }
    }

    #[method]
    fn _draw(&mut self, #[base] base: &Control) {
        let rect = base.get_rect();
        // tracing::debug!("{:?}",rect);
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
                        self.draw_pos(x as f32, y as f32),
                        "█".repeat(cell.width()),
                        Color::from_rgba(bg.0, bg.1, bg.2, bg.3),
                        -1,
                    );

                    base.draw_string(
                        &self.font,
                        self.draw_pos(x as f32, y as f32),
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
            self.draw_pos(cursor_pos.x as f32, cursor_pos.y as f32),
            "_",
            "",
            Color::from_rgba(1., 1., 1., 1.),
        );
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

    #[method]
    fn on_game_state(&mut self, #[base] base: TRef<Control>, state: GameState) -> Option<()> {
        tracing::debug!("on_game_state: {:?}", &state);
        match state {
            GameState::MissionComplete(msg) => {
                self.write("\n");
                self.write(&msg);
                self.prompt();
                base.update();
            }
            GameState::LevelChange(scene) => {
                self.clear();
                self.write_scene_message(scene);
                self.prompt();
                base.update();
            }
        };
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
