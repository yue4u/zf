use gdnative::prelude::*;
use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
};

use crate::vm::{CommandInput, CommandResult, CommandRun, CommandRunState, Parser, VMSignal};

#[derive(NativeClass, Debug, Default)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct VMManager {
    run_id: RefCell<u32>,
    cmd_id: RefCell<u32>,
    run_buffer: RefCell<Vec<CommandRun>>,
    result_buffer: RefCell<ResultBuffer>,
}

type ResultBuffer = HashMap<u32, CommandResult>;

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
    pub(crate) fn _ready(&self) {
        godot_print!("vm host ready");
        let mut runtime = zf_runtime::Runtime::new();
        let mut store = runtime.store(());

        let hello = zf_runtime::Func::wrap(&mut store, || {
            godot_print!("Calling back...");
            godot_print!("> hello from wasm!");
        });

        runtime.run(store, hello).unwrap();
    }

    #[method]
    pub(crate) fn on_cmd_entered(&self, #[base] base: &Node, text: String) -> Option<()> {
        godot_print!("on_cmd_entered: {text}!");
        base.emit_signal(VMSignal::OnCmdEntered, &[Variant::new(text.clone())]);

        let cmds = Parser::parse(text).ok()?;
        let id = self.run_id.replace_with(|&mut i| i + 1);
        let run = CommandRun {
            id,
            active_id: 0,
            cmds: cmds
                .into_iter()
                .map(|cmd| CommandInput {
                    cmd,
                    id: self.cmd_id.replace_with(|&mut i| i + 1),
                })
                .collect(),
            state: CommandRunState::Running,
        };
        let first = run.cmds.first()?.clone();
        self.run_buffer.borrow_mut().push(run);
        base.emit_signal(VMSignal::OnCmdParsed, &[Variant::new(first)]);
        Some(())
    }

    #[method]
    pub fn on_cmd_result(&self, #[base] base: &Node, result: CommandResult) -> Option<()> {
        godot_print!("receive_command_result: {}", result.id);

        let mut result_buffer = self.result_buffer.borrow_mut();
        result_buffer.insert(result.id, result);

        for run in self.run_buffer.borrow_mut().iter_mut() {
            process_cmd(base, &mut result_buffer, run);
        }

        Some(())
    }
}

fn process_cmd(
    base: &Node,
    result_buffer: &mut RefMut<ResultBuffer>,
    run: &mut CommandRun,
) -> Option<()> {
    let waiting = run.cmds.len();
    run.cmds = run
        .cmds
        .clone()
        .into_iter()
        .skip_while(|cmd| {
            if let Some(res) = result_buffer.get(&cmd.id) {
                base.emit_signal(VMSignal::OnCmdResult, &res.as_var());
                run.active_id += 1;
                return true;
            }
            false
        })
        .collect();

    if run.cmds.len() < waiting {
        let cmd = run.cmds.first()?;
        base.emit_signal(VMSignal::OnCmdParsed, &[Variant::new(cmd)]);
    }
    Some(())
}
