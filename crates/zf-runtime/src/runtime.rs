use std::io::Cursor;
use wasi_common::{pipe::WritePipe, WasiCtx};
use wasmtime::*;
use wasmtime_wasi::WasiCtxBuilder;
use zf_ffi::{CommandArgs, TaskListenableEvent, WeaponName};

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
// https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies
#[cfg(target_os = "linux")]
pub const SHELL_WASM: &[u8] = include_bytes!(env!("CARGO_BIN_FILE_ZF_SHELL"));

#[cfg(not(target_os = "linux"))]
// bin_deps does not compile except linux
pub const SHELL_WASM: &[u8] = include_bytes!(concat!(
    env!("CARGO_WORKSPACE_DIR"),
    "target/wasm32-wasi/release/zf-shell.wasm"
));

pub const SHELL_PRELOAD: &'static str = r#"
alias e = engine;
alias f = fire;
"#;

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

    pub fn cmds_available(&mut self) -> anyhow::Result<Vec<String>> {
        let cmds_raw = self.eval("help commands | get name | str join $'(char nl)'")?;
        let mut cmds = cmds_raw
            .lines()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();

        // provide extra task on
        cmds.append(
            &mut TaskListenableEvent::all()
                .iter()
                .map(|ev| format!("task on {ev}"))
                .collect::<Vec<String>>(),
        );

        // provide extra weapons
        cmds.append(
            &mut WeaponName::all()
                .iter()
                .map(|w| format!("fire {}", w.as_str()))
                .collect::<Vec<String>>(),
        );
        Ok(cmds)
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
