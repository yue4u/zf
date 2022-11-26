mod commands;
mod imports;
mod memory;
mod shell;

fn main() {
    let result = shell::eval_stateless(
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
    let result = shell::eval(line).map_err(|e| format!("{:?}", e));

    memory::alloc_encode(result)
}

#[test]
fn sanity() {
    let result = shell::eval_stateless(
        "[1 2 3] | math sum".to_string(), //
    );
    dbg!(&result);
    assert_eq!(result.ok(), Some("6".to_string()));
}

#[test]
fn state() {
    shell::eval(
        "alias the_freedom_to_run_the_program_as_you_wish_for_any_purpose = hi".to_string(), //
    )
    .unwrap();

    let result = shell::eval(
        "the_freedom_to_run_the_program_as_you_wish_for_any_purpose".to_string(), //
    );

    assert_eq!(result.ok(), Some("hi!".to_string()));
}
