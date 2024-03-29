use std::collections::VecDeque;

use gdnative::{
    api::{
        control::FocusMode,
        object::ConnectFlags,
        AudioStreamPlayer,
        // Font,
        DynamicFont,
        GlobalConstants,
        // InputEventMouseButton,
        ItemList,
        Particles2D,
        OS,
    },
    prelude::*,
};
use zf_ffi::{CommandArgs, TermCommand};
use zf_term::{TerminalSize, ZFTerm, ZF};

use crate::{
    common::{current_level, find_ref, PackedSceneRef, SceneLoader, StyledLabel},
    entities::{GameEvent, LevelHelper},
    managers::VM,
    refs::{self, HasPath},
    vm::{CommandInput, CommandResult, VMSignal},
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
    vm: Option<Instance<VM>>,
    term: ZFTerm,
    // HACK: This is very wrong and we should not doing this
    // term_scroll_offset: isize,
    process_state: ProcessState,
    buffer: String,
    history: VecDeque<String>,
    font: Ref<DynamicFont>,
    // font: Ref<Font>,
    base_cell_size: Vector2,
    completion_item_list: Ref<ItemList>,
    typing_particles: PackedSceneRef,
    audio_stream_player: Option<Ref<AudioStreamPlayer>>,
    bg_opacity: f32,
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
            .load(refs::path::assets::TERMINAL_FONT_TRES, "DynamicFont", false)
            .unwrap()
            .cast::<DynamicFont>()
            .unwrap();

        let writer = Box::new(TerminalWriter {});
        let base_cell_size = unsafe { font.assume_safe() }.get_string_size("W");
        let completion_item_list = ItemList::new().into_shared();
        Terminal {
            // seqno: 0,
            base: base.claim(),
            vm: None,
            font,
            process_state: ProcessState::Idle,
            buffer: String::new(),
            history: VecDeque::new(),
            base_cell_size,
            term: ZFTerm::new(writer, TerminalSize::default()),
            // term_scroll_offset: 0,
            completion_item_list,
            typing_particles: SceneLoader::load(refs::path::scenes::TYPING_PARTICLES).unwrap(),
            bg_opacity: 0.3,
            audio_stream_player: None,
        }
    }

    pub fn get_size(&self) -> TerminalSize {
        self.term.get_size()
    }

    #[method]
    fn resize(&mut self, #[base] base: TRef<Control>) {
        let term_size = calc_terminal_size(base, self.base_cell_size);
        self.term.resize(term_size);
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

        unsafe { base.get_node_as::<AudioStreamPlayer>("./SEAudioStreamPlayer") }.map(
            |audio_stream_player| self.audio_stream_player = Some(audio_stream_player.claim()),
        );

        let as_node = unsafe { base.get_node_as::<Node>(".")? };
        let vm_manager = find_ref::<VM, Node>(as_node)?;

        self.vm = vm_manager.cast_instance::<VM>().map(|vm| vm.claim());

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
                VMSignal::OnCmdParsed,
                base,
                VMSignal::OnCmdParsed,
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
        self.write(&current_level(&as_node).guide());
        self.prompt();

        Some(())
    }

    fn write(&mut self, data: &str) {
        self.term.advance_bytes(data);
    }

    fn write_with_effect(&mut self, data: &str) {
        self.create_typing_particles();
        self.term.advance_bytes(data);
    }

    fn prompt(&mut self) {
        use nu_ansi_term::Color::*;
        let before = match self.process_state {
            ProcessState::Error => Black.on(LightRed).paint(" "),
            _ /*     fmt     */ => Black.on(LightCyan).paint(" "),
        };
        self.write(&format!("\n{} > ", before));

        // repush buffer
        if !self.buffer.is_empty() {
            let buffer = self.buffer.clone();
            self.write(&buffer);
        }
    }

    fn clear(&mut self) {
        self.write("\x1b[2J\x1b[H");
    }

    fn create_typing_particles(&self) {
        let base = unsafe { self.base.assume_unique() };
        let cursor_pos = self.term.cursor_pos();
        let draw_pos = self.draw_pos(cursor_pos.x as f32, cursor_pos.y as f32);
        let typing_particles =
            SceneLoader::instance_as::<Particles2D>(&self.typing_particles).unwrap();
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
        let event = unsafe { event.assume_safe() };

        // skip if not pressed
        if !event.is_pressed() {
            return None;
        }

        // if let Some(mouse_button) = event.cast::<InputEventMouseButton>() {
        //     match mouse_button.button_index() {
        //         GlobalConstants::BUTTON_WHEEL_UP => {
        //             self.term.scroll_up(1);
        //             self.term_scroll_offset += 1;
        //             base.update()
        //         }
        //         GlobalConstants::BUTTON_WHEEL_DOWN => {
        //             self.term.scroll_down(1);
        //             self.term_scroll_offset -= 1;
        //             base.update()
        //         }
        //         _ => {}
        //     }
        //     return None;
        // };

        let event = event.cast::<InputEventKey>()?;

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
                        if self.history.len() == 50 {
                            self.history.pop_front();
                        }
                        self.history.push_back(lines.to_owned());
                        base.emit_signal(ENTER_SIGNAL, &[lines.to_variant()]);
                    }
                }
                self.buffer = "".to_string();
            }
            GlobalConstants::KEY_V if event.control() => {
                let clipboard = OS::godot_singleton().clipboard().to_string();
                let clipboard_str = clipboard.as_str();
                self.buffer.push_str(clipboard_str);
                self.write(clipboard_str);
            }
            GlobalConstants::KEY_C if event.control() => {
                self.buffer.clear();
                self.prompt();
            }
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
            GlobalConstants::KEY_UP if self.buffer.is_empty() => {
                // TODO: key up/down more than once
                if let Some(last) = self.history.iter().last() {
                    let text = last.clone();
                    self.write_with_effect(&text);
                    self.buffer = text;
                } else {
                    has_delta = false
                }
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
            let matched: Vec<String> = self
                .vm
                .as_ref()
                .and_then(|instance| {
                    unsafe { instance.assume_safe() }
                        .map(|vm, _| vm.complete(&self.buffer))
                        .ok()
                })
                .unwrap_or_default();
            let matched_len = matched.len();
            if !matched.is_empty() {
                cl_visible = true;

                if has_delta {
                    cl.set_focus_mode(FocusMode::NONE.into());

                    cl.clear();
                    let mut matched_max: usize = 0;
                    for item in matched.into_iter() {
                        matched_max = matched_max.max(item.len());
                        cl.add_item(item, GodotObject::null(), true);
                    }

                    cl.set_size(
                        Vector2 {
                            x: (matched_max + 1) as f32 * self.base_cell_size.x,
                            y: matched_len as f32 * self.base_cell_size.y,
                        },
                        false,
                    );

                    let cursor_pos = self.term.cursor_pos();
                    let cursor_pos_adjust = 0.5;
                    cl.set_position(
                        self.draw_pos(
                            (cursor_pos.x) as f32 + cursor_pos_adjust,
                            // subtract display len + 1
                            (cursor_pos.y - matched_len as i64) as f32 - cursor_pos_adjust,
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
            x: TERM_PADDING + x * self.base_cell_size.x,
            // position uses bottom-left so 2x here
            y: 2. * TERM_PADDING + y * self.base_cell_size.y,
        }
    }

    #[method]
    fn _draw(&mut self, #[base] base: &Control) {
        let rect = base.get_rect();
        // tracing::debug!("{:?}",rect);
        let color_palette = &self.term.get_config().color_palette();
        let anchor = Vector2 { x: 0., y: 0. };

        base.draw_rect(
            Rect2 {
                position: anchor,
                size: rect.size,
            },
            Color::from_rgba(0., 0., 0., self.bg_opacity),
            true,
            -1.,
            false,
        );

        let screen = self.term.screen_mut();

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
                // HACK: somehow when cell width > 1, next cell is a extra space. skip that.
                let mut skip = false;

                for cell in line.cells_mut() {
                    if skip {
                        skip = !skip;
                        if cell.str() == " " {
                            continue;
                        }
                    }
                    let fg = zf_term::Color::resolve_cell_fg_color(cell, color_palette);
                    let bg = zf_term::Color::resolve_cell_bg_color(cell, color_palette);
                    // base.draw_rect(
                    //     Rect2 {
                    //         position: Vector2 {
                    //             x: (x as f32) * self.base_cell_size.x,
                    //             y: (y as f32) * self.base_cell_size.y,
                    //         },
                    //         size: Vector2 {
                    //             x: cell.width() as f32 * self.base_cell_size.x,
                    //             y: self.base_cell_size.y,
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

                    if cell.width() > 1 {
                        skip = true
                    }
                    x += cell.width();
                }
            });

        // cursor
        let cursor_pos = self.term.cursor_pos();
        base.draw_char(
            &self.font,
            self.draw_pos(cursor_pos.x as f32, cursor_pos.y as f32),
            "_",
            "",
            Color::from_rgba(1., 1., 1., 1.),
        );
    }

    #[method]
    fn on_cmd_parsed(&mut self, #[base] base: TRef<Control>, input: CommandInput) {
        match input.cmd {
            CommandArgs::Term(TermCommand::Opacity(opacity)) => {
                self.bg_opacity = opacity;
                base.update();
            }
            _ => return,
        }
    }

    #[method]
    fn on_cmd_result(&mut self, #[base] base: &Control, result: CommandResult) -> Option<()> {
        let result = match result.result {
            Ok(result) => {
                self.process_state = ProcessState::Idle;
                result
            }
            Err(result) => {
                self.audio_stream_player.map(|player| {
                    unsafe { player.assume_safe() }.play(0.);
                });
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
    fn on_game_state(&mut self, #[base] base: TRef<Control>, state: GameEvent) -> Option<()> {
        tracing::debug!("on_game_state: {:?}", &state);
        match state {
            GameEvent::MissionComplete(msg) => {
                self.write("\n");
                self.write(&msg);
                self.write(&format!(
                    "type {} to continue",
                    StyledLabel::Code.paint("level next")
                ));
                self.prompt();
                base.update();
            }
            GameEvent::LevelChange(level) => {
                self.clear();
                self.write(&level.guide());
                self.prompt();
                base.update();
            }
            _ => {}
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
