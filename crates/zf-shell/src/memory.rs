/// https://radu-matei.com/blog/practical-guide-to-wasm-memory/
#[no_mangle]
pub fn alloc_string(len: usize) -> *mut u8 {
    let mut buf = String::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    return ptr;
}

pub unsafe fn string_from(parts: i64) -> String {
    let len = parts as i32;
    let ptr = (parts >> 32) as i32;
    String::from_raw_parts(
        ptr as *mut u8, //
        len as usize,
        len as usize,
    )
}
