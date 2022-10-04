mod runtime;
mod shell;

fn main() {
    shell::eval(std::env::args().nth(1).unwrap_or("help".to_string()));
}
