use zf_bridge::CommandBridge;
use zf_ffi::*;

use crate::memory;

pub fn zf_call(args: CommandBridge) -> String {
    let tag_out = memory::alloc_encode(args);
    unsafe {
        let tag_in = zf_cmd(tag_out);
        memory::string_from(tag_in)
    }
}
