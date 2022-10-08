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

#[test]
fn sanity() {
    let result = shell::eval(
        "[1 2 3] | math sum".to_string(), //
    );
    assert_eq!(result.ok(), Some("6".to_string()));
}
