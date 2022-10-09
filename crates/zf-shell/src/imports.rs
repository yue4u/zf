use zf_bridge::CommandBridge;

use crate::memory;

#[cfg(target_os = "wasi")]
#[link(wasm_import_module = "zf")]
extern "C" {
    /// args: tag of ZFCommandArgs
    /// return: tag of String
    fn zf_cmd(args: i64) -> i64;
}

#[cfg(not(target_os = "wasi"))]
/// placeholder to make tests compile
fn zf_cmd(_args: i64) -> i64 {
    0
}

pub fn zf_call(args: CommandBridge) -> String {
    let tag_out = memory::alloc_cmd_args(args);
    unsafe {
        let tag_in = zf_cmd(tag_out);
        memory::string_from(tag_in)
    }
}
