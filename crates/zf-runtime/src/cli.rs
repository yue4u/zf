use zf_runtime::{test_runtime, SHELL_PRELOAD};

fn main() -> anyhow::Result<()> {
    let mut runtime = test_runtime()?;
    runtime.eval(SHELL_PRELOAD).unwrap();

    let result = runtime.eval(std::env::args().nth(1).unwrap())?;
    println!("{result}");
    Ok(())
}
