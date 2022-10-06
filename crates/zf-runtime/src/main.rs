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
    use wasi_common::pipe::WritePipe;
    use wasmtime::{Engine, Linker, Module, Store};
    use wasmtime_wasi::WasiCtxBuilder;

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let stdout = WritePipe::new_in_memory();

    let wasi = WasiCtxBuilder::new()
        .stdout(Box::new(stdout.clone()))
        .args(&["".to_owned(), "[1 2 3] | math sum".to_string()])?
        .build();
    let mut store = Store::new(&engine, wasi);

    let shell_wasm = include_bytes!("../../target/wasm32-wasi/release/zf-shell.wasm");
    let module = Module::from_binary(&engine, shell_wasm)?;
    linker.module(&mut store, "", &module)?;
    linker
        .get_default(&mut store, "")?
        .typed::<(), (), _>(&store)?
        .call(&mut store, ())?;

    drop(store);

    let contents: Vec<u8> = stdout.try_into_inner().unwrap().into_inner();

    assert_eq!(String::from_utf8(contents).unwrap(), "6".to_string());
    Ok(())
}
