use gdnative::prelude::*;
use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
};

use crate::{
    common::find_ref,
    ui::CommandPalette,
    vm::{CommandInput, CommandResult, CommandRun, CommandRunState, Parser, VMSignal},
};

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
    pub(crate) fn new(_owner: &Node) -> Self {
        VMManager::default()
    }

    pub(crate) fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal(VMSignal::OnCmdEntered.as_str()).done();
        builder.signal(VMSignal::OnCmdParsed.as_str()).done();
        builder.signal(VMSignal::OnCmdResult.as_str()).done();
    }

    #[export]
    pub(crate) fn _ready(&self, owner: TRef<Node>) -> Option<()> {
        godot_print!("vm host ready");
        find_ref::<CommandPalette, Node>(owner)?
            .connect(
                "text_entered",
                owner,
                VMSignal::OnCmdEntered,
                VariantArray::new_shared(),
                0,
            )
            .expect("failed to connect line edit");

        Some(())
    }

    #[export]
    pub(crate) fn on_cmd_entered(&self, owner: &Node, text: String) -> Option<()> {
        godot_print!("on_cmd_entered: {text}!");
        owner.emit_signal(VMSignal::OnCmdEntered, &[Variant::new(text.clone())]);

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
        owner.emit_signal(VMSignal::OnCmdParsed, &[Variant::new(first)]);
        Some(())
    }

    #[export]
    pub fn on_cmd_result(&self, owner: &Node, result: CommandResult) -> Option<()> {
        godot_print!("receive_command_result: {}", result.id);

        let mut result_buffer = self.result_buffer.borrow_mut();
        result_buffer.insert(result.id, result);

        for run in self.run_buffer.borrow_mut().iter_mut() {
            process_cmd(owner, &mut result_buffer, run);
        }

        Some(())
    }
}

fn process_cmd(
    owner: &Node,
    result_buffer: &mut RefMut<ResultBuffer>,
    run: &mut CommandRun,
) -> Option<()> {
    godot_dbg!(&run.cmds);
    let waiting = run.cmds.len();
    run.cmds = run
        .cmds
        .clone()
        .into_iter()
        .skip_while(|cmd| {
            if let Some(res) = result_buffer.get(&cmd.id) {
                owner.emit_signal(VMSignal::OnCmdResult, &res.as_var());
                run.active_id += 1;
                return true;
            }
            false
        })
        .collect();

    if run.cmds.len() < waiting {
        let cmd = run.cmds.first()?;
        owner.emit_signal(VMSignal::OnCmdParsed, &[Variant::new(cmd)]);
    }
    Some(())
}
