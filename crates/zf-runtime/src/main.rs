use anyhow::Result;
mod runtime;

fn main() -> Result<()> {
    let mut runtime = runtime::Runtime::new();
    let mut store = runtime.store(());

    let hello = zf_runtime::Func::wrap(&mut store, || {
        print!("Calling back...");
        print!("> hello from wasm!");
    });

    runtime.run(store, hello)
}
