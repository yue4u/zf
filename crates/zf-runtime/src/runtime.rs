use std::io::Cursor;

use anyhow::Ok;
use wasi_common::{pipe::WritePipe, WasiCtx};
use wasmtime::*;
use wasmtime_wasi::WasiCtxBuilder;
use zf_ffi::CommandArgs;

pub use wasmtime::{Caller, Func, Store};

use crate::{decode_from_caller, memory};

pub struct Runtime<S> {
    linker: Linker<ExtendedStore<S>>,
    pub store: Store<ExtendedStore<S>>,
    instance: Instance,
    _stdout: WritePipe<Cursor<Vec<u8>>>,
    _stderr: WritePipe<Cursor<Vec<u8>>>,
}

pub const SHELL_MODULE: &'static str = "zf-shell";
pub const SHELL_WASM: &[u8] = include_bytes!(concat!(
    // https://github.com/rust-lang/cargo/issues/3946
    // we don't have `CARGO_WORKSPACE_DIR` at home
    "../../../",
    "target/wasm32-wasi/release/zf-shell.wasm"
));

pub const SHELL_PRELOAD: &'static str = r#"
alias g = game;
alias t = game tutorial;
alias e = engine;
alias f = fire;
"#;

// generated via `help commands` for naive completions.
// this only cmd name contains for now
// it's better to do this in the shell but just much easier outside
#[allow(dead_code)]
const SHELL_CMDS: &'static str = include_str!("./cmds.txt");

#[allow(dead_code)]
pub fn cmds() -> Vec<&'static str> {
    SHELL_CMDS.lines().collect()
}

#[allow(dead_code)]
pub fn strip_ansi(input: impl std::fmt::Display) -> String {
    String::from_utf8_lossy(&strip_ansi_escapes::strip(input.to_string()).unwrap()).to_string()
}

pub struct ExtendedStore<T> {
    pub ext: T,
    pub wasi: WasiCtx,
}

impl<S> Runtime<S> {
    pub fn init<F>(store_ext: S, prepare: F) -> anyhow::Result<Runtime<S>>
    where
        F: FnOnce(&mut Linker<ExtendedStore<S>>) -> anyhow::Result<()>,
    {
        let engine = Engine::default();
        let mut linker = Linker::<ExtendedStore<S>>::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi)?;

        let stdout = WritePipe::new_in_memory();
        let stderr = WritePipe::new_in_memory();

        let wasi = WasiCtxBuilder::new()
            .stdout(Box::new(stdout.clone()))
            .stderr(Box::new(stderr.clone()))
            .build();
        let mut store = Store::new(
            &engine,
            ExtendedStore {
                ext: store_ext,
                wasi,
            },
        );
        let zf_shell_module = Module::from_binary(&engine, SHELL_WASM)?;

        prepare(&mut linker)?;

        let instance = linker.instantiate(&mut store, &zf_shell_module)?;

        linker.instance(&mut store, SHELL_MODULE, instance)?;

        Ok(Runtime {
            linker,
            store,
            instance,
            _stdout: stdout,
            _stderr: stderr,
        })
    }

    pub fn eval(&mut self, input: impl Into<String>) -> anyhow::Result<String> {
        let memory = self
            .instance
            .get_export(&mut self.store, "memory")
            .unwrap()
            .into_memory()
            .unwrap();

        let input =
            memory::write_string_from_host(self.instance, &mut self.store, &memory, input.into());
        let tag = self
            .linker
            .get(&mut self.store, SHELL_MODULE, "eval")
            .expect("expect eval function exist")
            .into_func()
            .expect("expect eval function ok")
            .typed::<i64, i64, _>(&self.store)?
            .call(&mut self.store, input)?;

        memory::decode_from_host::<_, Result<String, String>>(&mut self.store, &memory, tag)
            .map_err(|e| anyhow::Error::msg(e))

        // let Runtime {
        //     store,
        //     stderr,
        //     stdout,
        //     ..
        // } = self;

        // drop(store);

        // let stdout: Vec<u8> = stdout.try_into_inner().unwrap().into_inner();
        // let stderr: Vec<u8> = stderr.try_into_inner().unwrap().into_inner();
        // // if stderr.is_empty() {
        // // return Ok(String::from_utf8(stdout)?.to_string());
        // // }
        // let out = String::from_utf8(stdout)?.to_string();
        // let err = String::from_utf8(stderr)?.to_string();

        // dbg!(&out, &err);
    }
}

pub struct TestStore {
    pub last_cmd_call: Option<zf_ffi::CommandArgs>,
}

pub fn test_runtime() -> anyhow::Result<Runtime<TestStore>> {
    let store = TestStore {
        last_cmd_call: None,
    };
    let runtime = Runtime::init(store, |linker| -> anyhow::Result<()> {
        linker.func_wrap(
            "zf",
            "zf_cmd",
            |mut caller: Caller<'_, ExtendedStore<TestStore>>, tag: i64| -> i64 {
                let cmd: CommandArgs = decode_from_caller(&mut caller, tag);
                dbg!(&cmd);
                let ret = match &cmd {
                    &zf_ffi::CommandArgs::Mystery => {
                        memory::write_string_with_caller(&mut caller, "🌈 it works!!".to_owned())
                    }
                    _ => 0,
                };
                caller.data_mut().ext.last_cmd_call = Some(cmd);
                ret
            },
        )?;

        linker.func_wrap(
            "zf",
            "zf_terminal_size",
            |mut _caller: Caller<'_, ExtendedStore<TestStore>>| -> i64 {
                zf_ffi::memory::Tag::into(80, 20)
            },
        )?;

        Ok(())
    })?;

    Ok(runtime)
}
