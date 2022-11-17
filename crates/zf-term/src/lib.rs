use std::sync::Arc;
use wezterm_term::Terminal;
use wezterm_term::{color::ColorPalette, TerminalConfiguration, TerminalSize};

#[derive(Debug)]
struct ZFTermConfig {
    scrollback: usize,
}
impl TerminalConfiguration for ZFTermConfig {
    fn scrollback_size(&self) -> usize {
        self.scrollback
    }

    fn color_palette(&self) -> ColorPalette {
        ColorPalette::default()
    }
}
pub struct ZFTermState {
    // pub writer: Box<ZFTermWriter>,
    pub term: Terminal,
}

impl ZFTermState {
    pub fn new(writer: Box<dyn std::io::Write + Send>) -> Self {
        let size = TerminalSize {
            rows: 40,
            cols: 100,
            pixel_width: 16,
            pixel_height: 16,
            dpi: 100,
        };
        let config = Arc::new(ZFTermConfig { scrollback: 20 });
        let term = Terminal::new(size, config, "zf-shell", "0.0.0", writer);
        Self { term }
    }
}

#[cfg(test)]
struct TestWriter;

#[cfg(test)]
impl std::io::Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        dbg!(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        dbg!("flush");
        Ok(())
    }
}

#[test]
fn test() {
    let mut t = ZFTermState::new(Box::new(TestWriter));
    t.term.send_paste("text").unwrap();
    t.term
        .advance_bytes("\u{001b}[30m A \u{001b}[31m B \u{001b}[32m C \u{001b}[33m D \u{001b}[0m");
    dbg!(&t.term.screen_mut().line_mut(0).cells_mut());
}
