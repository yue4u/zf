#[link(wasm_import_module = "zf")]
extern "C" {
    pub fn game_start() -> i64;
    pub fn game_end() -> i64;
    pub fn game_menu() -> i64;

    pub fn engine(args: i64);
}
