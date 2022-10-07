use gdnative::prelude::*;
use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
};

use crate::vm::{Command, CommandInput, CommandResult, Parser, Process, ProcessState, VMSignal};

#[derive(NativeClass, Debug, Default)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct VMManager {
    process_id: RefCell<u32>,
    cmd_id: RefCell<u32>,
    process_buffer: RefCell<Vec<Process>>,
    result_buffer: RefCell<ResultBuffer>,
    // TODO: more pts
}

type ResultBuffer = HashMap<u32, CommandResult>;

struct VMData<'a> {
    base: &'a Node,
}

#[methods]
impl VMManager {
    pub(crate) fn new(_base: &Node) -> Self {
        VMManager::default()
    }

    pub(crate) fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal(VMSignal::OnCmdEntered.as_str()).done();
        builder.signal(VMSignal::OnCmdParsed.as_str()).done();
        builder.signal(VMSignal::OnCmdResult.as_str()).done();
    }

    #[method]
    pub(crate) fn _ready(&self, #[base] base: &Node) {
        godot_print!("vm host ready");

        // let mut runtime = zf_runtime::Runtime::new();
        // let vm_data = VMData { base };
        // let mut store = runtime.store(vm_data);
        // let hello = zf_runtime::Func::wrap(&mut store, |caller: zf_runtime::Caller<'_, VMData>| {
        //     godot_print!("Calling back...");
        //     godot_print!("> hello from wasm!");
        //     godot_print!("> current path is {:?}", caller.data().base.get_path());
        // });

        // runtime
        //     .run(&mut store, &[hello.into()], zf_runtime::HELLO_WAT)
        //     .unwrap();
    }

    #[method]
    pub(crate) fn on_cmd_entered(&self, #[base] base: &Node, text: String) -> Option<()> {
        godot_print!("on_cmd_entered: {text}!");
        base.emit_signal(VMSignal::OnCmdEntered, &[Variant::new(text.clone())]);

        // let cmds = match Parser::parse(text) {
        //     Ok(cmds) => cmds,
        //     Err(e) => {
        //         self.on_cmd_result(
        //             base,
        //             CommandResult {
        //                 id: 0,
        //                 result: Err(format!("failed to parse command: {:#?}", e)),
        //             },
        //         );
        //         return None;
        //     }
        // };
        // let id = self.process_id.replace_with(|&mut i| i + 1);
        // let process = Process {
        //     id,
        //     active_id: 0,
        //     cmds: cmds
        //         .into_iter()
        //         .map(|cmd| CommandInput {
        //             cmd,
        //             id: self.cmd_id.replace_with(|&mut i| i + 1),
        //         })
        //         .collect(),
        //     state: ProcessState::Running,
        // };
        // let first = process.cmds.first()?.clone();
        // self.process_buffer.borrow_mut().push(process);
        // base.emit_signal(VMSignal::OnCmdParsed, &[Variant::new(first)]);
        let result = CommandResult {
            id: 0,
            result: zf_runtime::Runtime::eval(text).map_err(|e| e.to_string()),
        };
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
