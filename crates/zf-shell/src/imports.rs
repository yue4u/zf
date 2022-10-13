use zf_bridge::CommandBridge;

use crate::memory;

#[cfg(target_os = "wasi")]
#[link(wasm_import_module = "zf")]
extern "C" {
    /// args: tag of ZFCommandArgs
    /// return: tag of String
    fn zf_cmd(args: i64) -> i64;
    // TODO: terminal size
    fn _terminal_size() -> u32;
}

#[cfg(not(target_os = "wasi"))]
/// placeholder to make tests compile
fn zf_cmd(_args: i64) -> i64 {
    0
}

// #[cfg(not(target_os = "wasi"))]
// /// placeholder to make tests compile
// fn _terminal_size() -> u32 {
//     0
// }

// pub fn terminal_size() -> Option<(u16, u16)> {
//     let size = _terminal_size();
//     let width = (size >> 16) as u16;
//     let height = (size & 0xFFFF) as u16;
//     Some((width, height))
// }

pub fn zf_call(args: CommandBridge) -> String {
    let tag_out = memory::alloc_cmd_args(args);
    unsafe {
        let tag_in = zf_cmd(tag_out);
        memory::string_from(tag_in)
    }
}
