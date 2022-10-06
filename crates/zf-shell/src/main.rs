mod commands;
#[macro_use]
mod shell;

fn main() {
    let result = shell::eval(
        std::env::args().nth(1).unwrap_or("help".to_string()), //
    );
    println!("{:?}", result);
}

#[test]
fn sanity() {
    let result = shell::eval(
        "[1 2 3] | math sum".to_string(), //
    );
    assert_eq!(result.ok(), Some("6".to_string()));
}
