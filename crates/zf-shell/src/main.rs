mod commands;
#[macro_use]
mod shell;
mod imports;
mod memory;

fn main() {
    let result = shell::eval(
        std::env::args().nth(1).unwrap_or("".to_string()), //
    );
    match result {
        Ok(inner) => print!("{}", inner),
        Err(e) => eprint!("{:?}", e),
    };
}

#[no_mangle]
pub fn eval(input: i64) -> i64 {
    let line = unsafe { memory::string_from(input) };
    let result = shell::eval(line);
    let out = match result {
        Ok(inner) => format!("{}", inner),
        Err(e) => format!("{:?}", e),
    };
    memory::alloc_string_inside(out) as i64
}

#[test]
fn sanity() {
    let result = shell::eval(
        "[1 2 3] | math sum".to_string(), //
    );
    assert_eq!(result.ok(), Some("6".to_string()));
}
