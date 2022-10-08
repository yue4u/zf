use anyhow::Result;
mod runtime;

fn main() -> Result<()> {
    let result = runtime::Runtime::eval(std::env::args().nth(1).unwrap())?;
    println!("{result}");
    Ok(())
}

#[test]
fn sanity() -> anyhow::Result<()> {
    use runtime::Runtime;

    assert_eq!(
        Runtime::eval("[1 2 3] | math sum").unwrap(),
        "6".to_string()
    );
    Ok(())
}
