#[link(wasm_import_module = "zf")]
extern "C" {
    // multi value wasm compilation does not work yet
    // so conbine two i32 to i64
    // https://github.com/rust-lang/rust/issues/73755
    pub fn game_start() -> i64;
    pub fn game_end() -> i64;
    pub fn game_menu() -> i64;
}
