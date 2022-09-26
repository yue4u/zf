use anyhow::Result;
use wasmtime::*;
pub use wasmtime::{Caller, Func, Store};

pub struct Runtime {
    engine: Engine,
}

pub const HELLO_WAT: &[u8] = include_bytes!("./bin/hello.wat");

impl Runtime {
    pub fn new() -> Self {
        let engine = Engine::default();
        Self { engine }
    }

    pub fn store<T>(&mut self, data: T) -> Store<T> {
        Store::new(&self.engine, data)
    }

    pub fn run<T>(&self, store: &mut Store<T>, imports: &[Extern], binary: &[u8]) -> Result<()> {
        let module = Module::new(&self.engine, binary)?;

        let instance = Instance::new(&mut *store, &module, &imports)?;

        let run = instance.get_typed_func::<(), (), _>(&mut *store, "run")?;

        run.call(&mut *store, ())?;

        Ok(())
    }
}
