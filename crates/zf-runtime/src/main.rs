use anyhow::Result;
mod runtime;

fn main() -> Result<()> {
    // let mut runtime = runtime::Runtime::new();
    // let mut store = runtime.store(());

    // let hello = runtime::Func::wrap(&mut store, || {
    //     println!("Calling back...");
    //     println!("> hello from wasm!");
    // });

    // runtime.run(&mut store, &[hello.into()], runtime::HELLO_WAT)
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
