use wasi_common::WasiCtx;
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
        let stderr = WritePipe::new_in_memory();

        let wasi = WasiCtxBuilder::new()
            .stdout(Box::new(stdout.clone()))
            .stderr(Box::new(stderr.clone()))
            .args(&["".to_owned(), input.into()])?
            .build();
        let mut store = Store::new(&engine, wasi);
        let zf_shell_module = Module::from_binary(&engine, SHELL_WASM)?;

        let zf_shell_instance = linker
            .func_wrap(
                "zf",
                "game_start",
                |mut caller: Caller<'_, WasiCtx>| -> i64 {
                    let content = "🌈 it works!".to_owned();
                    let content = content.as_bytes();
                    let mem = caller.get_export("memory").unwrap().into_memory().unwrap();
                    let alloc_string = caller
                        .get_export("alloc_string")
                        .unwrap()
                        .into_func()
                        .unwrap();

                    let mut store = caller.as_context_mut();
                    let len = content.len() as i32;
                    let ptr = alloc_string
                        .typed::<i32, i32, _>(&mut store)
                        .unwrap()
                        .call(&mut store, len)
                        .unwrap();

                    mem.write(&mut store, ptr as usize, content).unwrap();

                    debug_assert_eq!(
                        &mem.data(&store)[ptr as usize..ptr as usize + len as usize],
                        content
                    );
                    (ptr as i64) << 32 | (len as i64)
                },
            )?
            .instantiate(&mut store, &zf_shell_module)?;

        linker.instance(&mut store, "zf-shell", zf_shell_instance)?;
        linker
            .get_default(&mut store, "zf-shell")?
            .typed::<(), (), _>(&store)?
            .call(&mut store, ())?;

        drop(store);

        let stdout: Vec<u8> = stdout.try_into_inner().unwrap().into_inner();
        let stderr: Vec<u8> = stderr.try_into_inner().unwrap().into_inner();
        if stderr.is_empty() {
            return Ok(String::from_utf8(stdout)?.to_string());
        }
        return String::from_utf8(stderr).map_err(Into::into);
    }
}
