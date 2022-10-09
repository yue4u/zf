use zf_bridge::ZFCommandArgs;

use crate::memory;

#[link(wasm_import_module = "zf")]
extern "C" {
    /// args: tag of ZFCommandArgs
    /// return: tag of String
    fn zf_cmd(args: i64) -> i64;
}

pub fn zf_call(args: ZFCommandArgs) -> String {
    let tag_out = memory::alloc_cmd_args(args);
    unsafe {
        let tag_in = zf_cmd(tag_out);
        memory::string_from(tag_in)
    }
}
