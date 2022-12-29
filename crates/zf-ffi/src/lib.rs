pub mod commands;
pub mod memory;
pub mod results;

pub use bincode::*;
pub use commands::*;
pub use results::*;

#[link(wasm_import_module = "zf")]
extern "C" {
    /// args: tag of ZFCommandArgs
    /// return: tag of String
    fn zf_cmd(args: i64) -> i64;
    /// return: combined width & height
    fn zf_terminal_size() -> i64;
}

// from https://github.com/eminence/terminal-size/blob/271d23e96d247245e0a152e147010129a2c8ca37/src/lib.rs
// LICENSE: https://github.com/eminence/terminal-size/blob/271d23e96d247245e0a152e147010129a2c8ca37/LICENSE-MIT
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Width(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Height(pub u16);

pub fn terminal_size() -> Option<(Width, Height)> {
    let tag = unsafe { zf_terminal_size() };
    let (w, h) = memory::Tag::from(tag);
    Some((Width(w as u16), Height(h as u16)))
}

pub fn cmd_legacy(args: commands::CommandArgs) -> String {
    let tag_out = memory::alloc_encode(args);
    unsafe {
        let tag_in = zf_cmd(tag_out);
        memory::string_from(tag_in)
    }
}

pub fn cmd(args: commands::CommandArgs) -> CommandResults {
    let tag_out = memory::alloc_encode(args);
    unsafe {
        let tag_in = zf_cmd(tag_out);
        memory::result_from(tag_in)
    }
}