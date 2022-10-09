use zf_bridge::{config, encode_to_vec, Tag};

use zf_bridge::CommandBridge;

/// https://radu-matei.com/blog/practical-guide-to-wasm-memory/
#[no_mangle]
pub fn alloc_string(len: usize) -> *mut u8 {
    let mut buf = String::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

pub fn alloc_string_inside(mut string: String) -> i64 {
    string.shrink_to_fit();
    let ptr = string.as_mut_ptr();
    let len = string.len() as i32;
    std::mem::forget(string);

    Tag::into(ptr as i32, len)
}

pub unsafe fn string_from(tag: i64) -> String {
    let (ptr, len) = Tag::from(tag);
    String::from_raw_parts(
        ptr as *mut u8, //
        len as usize,
        len as usize,
    )
}

pub fn alloc_cmd_args(args: CommandBridge) -> i64 {
    let config = config::standard();
    let mut vec = encode_to_vec(args, config).unwrap();
    let ptr = vec.as_mut_ptr();
    let len = vec.len() as i32;

    std::mem::forget(vec);

    Tag::into(ptr as i32, len)
}
