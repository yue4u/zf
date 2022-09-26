use anyhow::Result;
use wasmtime::*;
pub use wasmtime::{Func, Store};

pub struct Runtime {
    engine: Engine,
}

impl Runtime {
    pub fn new() -> Self {
        let engine = Engine::default();
        Self { engine }
    }

    pub fn store<T>(&mut self, data: T) -> Store<T> {
        Store::new(&self.engine, data)
    }

    // pub fn run(&self, binary: &[u8]) -> Result<()> {
    pub fn run(&self, mut store: Store<()>, hello: Func) -> Result<()> {
        let binary = include_bytes!("./bin/hello.wat");
        let module = Module::new(&self.engine, binary)?;

        // let imports = [hello_func.into()];
        let imports = [hello.into()];
        let instance = Instance::new(&mut store, &module, &imports)?;

        let run = instance.get_typed_func::<(), (), _>(&mut store, "run")?;

        run.call(&mut store, ())?;

        Ok(())
    }
}
