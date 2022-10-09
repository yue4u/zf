use gdnative::prelude::*;
use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
};

use crate::vm::{Command, CommandInput, CommandResult, IntoCommand, Process, VMSignal};

use zf_runtime::{cmd_args_from_caller, Caller, ExtendedStore, Runtime};

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct VMManager {
    // process_id: RefCell<u32>,
    // cmd_id: RefCell<u32>,
    process_buffer: RefCell<Vec<Process>>,
    result_buffer: RefCell<ResultBuffer>,
    // TODO: more pts
    runtime: Option<Runtime<VMData>>,
}

type ResultBuffer = HashMap<u32, CommandResult>;

struct VMData {
    cmd_id: u32,
    base: Ref<Node>,
}

#[methods]
impl VMManager {
    pub(crate) fn new(_base: &Node) -> Self {
        VMManager {
            // process_id: RefCell::new(0),
            // cmd_id: RefCell::new(0),
            process_buffer: RefCell::new(vec![]),
            result_buffer: RefCell::new(HashMap::new()),
            runtime: None,
        }
    }

    pub(crate) fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal(VMSignal::OnCmdEntered.as_str()).done();
        builder.signal(VMSignal::OnCmdParsed.as_str()).done();
        builder.signal(VMSignal::OnCmdResult.as_str()).done();
    }

    #[method]
    pub(crate) fn _ready(&mut self, #[base] base: TRef<Node>) {
        godot_print!("vm host ready");

        let vm_data = VMData {
            cmd_id: 0,
            base: base.claim(),
        };

        self.runtime = Some(
            Runtime::init(vm_data, |linker| -> anyhow::Result<()> {
                linker.func_wrap(
                    "zf",
                    "zf_cmd",
                    |mut caller: Caller<'_, ExtendedStore<VMData>>, tag: i64| -> i64 {
                        let cmd = cmd_args_from_caller(&mut caller, tag);
                        godot_dbg!(&cmd);
                        fire_and_forget(&mut caller.data_mut().ext, cmd.into_command());
                        0
                    },
                )?;

                Ok(())
            })
            .unwrap(),
        )
    }

    #[method]
    pub(crate) fn on_cmd_entered(&mut self, #[base] base: &Node, text: String) -> Option<()> {
        let runtime = self.runtime.as_mut()?;
        godot_print!("on_cmd_entered: {text}!");
        base.emit_signal(VMSignal::OnCmdEntered, &[Variant::new(text.clone())]);

        let result = runtime.eval(text).map_err(|e| e.to_string());
        let id = runtime.store.data_mut().ext.cmd_id + 1;
        runtime.store.data_mut().ext.cmd_id = id;
        let result = CommandResult { id, result };
        godot_dbg!(&result);
        base.emit_signal(VMSignal::OnCmdResult, &result.as_var());

        Some(())
    }

    #[method]
    pub fn on_cmd_result(&self, #[base] base: &Node, result: CommandResult) -> Option<()> {
        godot_print!("receive_command_result: {}", result.id);

        let mut result_buffer = self.result_buffer.borrow_mut();
        base.emit_signal(VMSignal::OnCmdResult, &result.as_var());
        result_buffer.insert(result.id, result);

        for process in self.process_buffer.borrow_mut().iter_mut() {
            run(base, &mut result_buffer, process);
        }

        Some(())
    }
}

fn fire_and_forget(vm_data: &VMData, cmd: Command) {
    godot_dbg!(&cmd);
    unsafe { vm_data.base.assume_safe() }.emit_signal(
        VMSignal::OnCmdParsed,
        &[CommandInput {
            id: vm_data.cmd_id,
            cmd,
        }
        .to_variant()],
    );
}

fn run(base: &Node, result_buffer: &mut RefMut<ResultBuffer>, process: &mut Process) -> Option<()> {
    let waiting = process.cmds.len();
    process.cmds = process
        .cmds
        .clone()
        .into_iter()
        .skip_while(|cmd| {
            if let Some(_res) = result_buffer.get(&cmd.id) {
                process.active_id += 1;
                return true;
            }
            false
        })
        .collect();

    if process.cmds.len() < waiting {
        let cmd = process.cmds.first()?;
        base.emit_signal(VMSignal::OnCmdParsed, &[Variant::new(cmd)]);
    }
    Some(())
}
