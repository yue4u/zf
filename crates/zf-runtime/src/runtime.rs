use wasmtime::*;
pub use wasmtime::{Caller, Func, Store};

pub struct Runtime {
    // engine: Engine,
}

// pub const HELLO_WAT: &[u8] = include_bytes!("./bin/hello.wat");
pub const SHELL_WASM: &[u8] = include_bytes!("../../target/wasm32-wasi/release/zf-shell.wasm");

impl Runtime {
    // pub fn new() -> Self {
    //     let engine = Engine::default();
    //     Self { engine }
    // }

    // pub fn store<T>(&mut self, data: T) -> Store<T> {
    //     Store::new(&self.engine, data)
    // }

    // pub fn run<T>(&self, store: &mut Store<T>, imports: &[Extern], binary: &[u8]) -> Result<()> {
    //     let module = Module::new(&self.engine, binary)?;

    //     let instance = Instance::new(&mut *store, &module, &imports)?;

    //     let run = instance.get_typed_func::<(), (), _>(&mut *store, "run")?;

    //     run.call(&mut *store, ())?;

    //     Ok(())
    // }

    pub fn eval(input: impl Into<String>) -> anyhow::Result<String> {
        use wasi_common::pipe::WritePipe;
        use wasmtime_wasi::WasiCtxBuilder;

        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

        let stdout = WritePipe::new_in_memory();

        let wasi = WasiCtxBuilder::new()
            .stdout(Box::new(stdout.clone()))
            .args(&["".to_owned(), input.into()])?
            .build();
        let mut store = Store::new(&engine, wasi);

        let module = Module::from_binary(&engine, SHELL_WASM)?;
        linker.module(&mut store, "", &module)?;
        linker
            .get_default(&mut store, "")?
            .typed::<(), (), _>(&store)?
            .call(&mut store, ())?;

        drop(store);

        let contents: Vec<u8> = stdout.try_into_inner().unwrap().into_inner();
        String::from_utf8(contents).map_err(Into::into)
    }
}
