use anyhow::Result;
mod runtime;

fn main() -> Result<()> {
    let mut runtime = runtime::Runtime::new();
    let mut store = runtime.store(());

    let hello = runtime::Func::wrap(&mut store, || {
        println!("Calling back...");
        println!("> hello from wasm!");
    });

    runtime.run(&mut store, &[hello.into()], runtime::HELLO_WAT)
}

#[test]
fn sanity() -> anyhow::Result<()> {
    use wasmtime::{Engine, Linker, Module, Store};
    use wasmtime_wasi::WasiCtxBuilder;

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let wasi = WasiCtxBuilder::new()
        .args(&["".to_owned(), "[1 2 3 ] | math sum".to_string()])?
        .build();
    let mut store = Store::new(&engine, wasi);

    let shell_wasm = include_bytes!("../../target/wasm32-wasi/debug/zf-shell.wasm");
    let module = Module::from_binary(&engine, shell_wasm)?;
    linker.module(&mut store, "", &module)?;
    linker
        .get_default(&mut store, "")?
        .typed::<(), (), _>(&store)?
        .call(&mut store, ())?;
    Ok(())
}
