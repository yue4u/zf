use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use termwiz::escape::csi::{Edit, Mode, TerminalMode, TerminalModeCode, CSI};
use wezterm_term::Terminal;
// re-exporting building components from wezterm_term
pub use wezterm_color_types::*;
pub use wezterm_term::{color::*, *};

#[derive(Debug)]
struct ZFTermConfig {
    scrollback: usize,
}
impl TerminalConfiguration for ZFTermConfig {
    fn scrollback_size(&self) -> usize {
        self.scrollback
    }

    fn color_palette(&self) -> ColorPalette {
        ColorPalette {
            foreground: SrgbaTuple(1., 1., 1., 1.),
            background: SrgbaTuple(0., 0., 0., 0.),
            ..Default::default()
        }
    }
}

pub struct Color;

impl Color {
    pub fn resolve_cell_fg_color(cell: &Cell, palette: &color::ColorPalette) -> SrgbaTuple {
        let attrs = cell.attrs();
        let fg = cell.attrs().foreground();
        match fg {
            color::ColorAttribute::Default => palette.resolve_fg(fg),
            color::ColorAttribute::PaletteIndex(idx) if idx < 8 => {
                // For compatibility purposes, switch to a brighter version
                // of one of the standard ANSI colors when Bold is enabled.
                // This lifts black to dark grey.
                let idx = if attrs.intensity() == Intensity::Bold {
                    idx + 8
                } else {
                    idx
                };

                palette.resolve_fg(color::ColorAttribute::PaletteIndex(idx))
            }
            _ => palette.resolve_fg(fg),
        }
    }

    pub fn resolve_cell_bg_color(cell: &Cell, palette: &color::ColorPalette) -> SrgbaTuple {
        let attrs = cell.attrs();
        let bg = cell.attrs().background();
        match bg {
            color::ColorAttribute::Default => palette.resolve_bg(bg),
            color::ColorAttribute::PaletteIndex(idx) if idx < 8 => {
                // For compatibility purposes, switch to a brighter version
                // of one of the standard ANSI colors when Bold is enabled.
                // This lifts black to dark grey.
                let idx = if attrs.intensity() == Intensity::Bold {
                    idx + 8
                } else {
                    idx
                };

                palette.resolve_bg(color::ColorAttribute::PaletteIndex(idx))
            }
            _ => palette.resolve_bg(bg),
        }
    }
}

pub struct ZFTerm {
    // pub writer: Box<ZFTermWriter>,
    pub term: Terminal,
}

impl Deref for ZFTerm {
    type Target = Terminal;

    fn deref(&self) -> &Self::Target {
        &self.term
    }
}

impl DerefMut for ZFTerm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.term
    }
}

impl ZFTerm {
    pub fn new(writer: Box<dyn std::io::Write + Send>, size: TerminalSize) -> Self {
        let config = Arc::new(ZFTermConfig { scrollback: 1000 });
        let mut term = Terminal::new(size, config, "zf-shell", "0.0.0", writer);
        // showhow this is needed to set cursor for LF
        let automatic_newline = CSI::Mode(Mode::SetMode(TerminalMode::Code(
            TerminalModeCode::AutomaticNewline,
        )));
        term.advance_bytes(automatic_newline.to_string());
        Self { term }
    }

    pub fn scroll_up(&mut self, n: u32) {
        self.advance_bytes(CSI::Edit(Edit::ScrollUp(n)).to_string());
    }

    pub fn scroll_down(&mut self, n: u32) {
        self.advance_bytes(CSI::Edit(Edit::ScrollDown(n)).to_string());
    }
}

#[cfg(test)]
struct TestWriter;

#[cfg(test)]
impl std::io::Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // dbg!(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // dbg!("flush");
        Ok(())
    }
}

pub const ZF: &'static str = include_str!("./weclome.ansi");

#[cfg(test)]
use expect_test::{expect, Expect};

#[cfg(test)]
fn check(actual: impl ToString, expect: Expect) {
    expect.assert_eq(&actual.to_string());
}

#[cfg(test)]
fn check_screen(mut t: ZFTerm, expect: Expect) {
    let mut buf = String::new();
    t.term.screen_mut().for_each_phys_line_mut(|idx, line| {
        buf.push_str(&format!("{:02}) |", idx));
        buf.push_str(
            &line
                .cells_mut()
                .into_iter()
                .map(|cell| if cell.width() >= 1 { cell.str() } else { "" })
                .collect::<String>(),
        );
        buf.push_str("\n");
    });

    check(&buf, expect);
}

#[test]
fn sanity() {
    let mut t = ZFTerm::new(
        Box::new(TestWriter),
        TerminalSize {
            rows: 2,
            ..Default::default()
        },
    );
    t.term.send_paste("text").unwrap();
    t.term
        .advance_bytes("\u{001b}[30m A \u{001b}[31m B \u{001b}[32m C \u{001b}[33m D \u{001b}[0m");
    check_screen(
        t,
        expect![[r#"
            00) | A  B  C  D 
            01) |
        "#]],
    );
}

#[test]
fn multiline() {
    let mut t = ZFTerm::new(
        Box::new(TestWriter),
        TerminalSize {
            rows: 7,
            ..Default::default()
        },
    );
    t.term.advance_bytes(ZF);
    check_screen(
        t,
        expect![[r#"
            00) |___          _        _     _______
            01) || _ \_ _ ___ (_)___ __| |_  |_  / __|
            02) ||  _/ '_/ _ \| / -_) _|  _|  / /| _|
            03) ||_| |_| \___// \___\__|\__| /___|_|
            04) |           |__/
            05) |
            06) |Weclome to zf console!
            07) |
        "#]],
    )
}
