use gdnative::prelude::*;
use std::{cell::RefCell, collections::HashMap};

use crate::{
    common::find_ref,
    ui::CommandPalette,
    vm::{CommandInput, CommandResult, CommandRun, CommandRunState, Parser},
};

#[derive(NativeClass, Debug, Default)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct VMManger {
    run_id: RefCell<u32>,
    cmd_id: RefCell<u32>,
    run_buffer: RefCell<Vec<CommandRun>>,
    result_buffer: RefCell<ResultBuffer>,
}

type ResultBuffer = HashMap<u32, CommandResult>;

#[methods]
impl VMManger {
    pub(crate) fn new(_owner: &Node) -> Self {
        VMManger::default()
    }

    pub(crate) fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal("on_cmd_entered").done();
        builder.signal("on_cmd_parsed").done();
        builder.signal("on_cmd_result").done();
    }

    #[export]
    pub(crate) fn _ready(&self, owner: TRef<Node>) -> Option<()> {
        godot_print!("vm host ready");
        find_ref::<CommandPalette, Node>(owner)?
            .connect(
                "text_entered",
                owner,
                "on_cmd_entered",
                VariantArray::new_shared(),
                0,
            )
            .expect("failed to connect line edit");

        Some(())
    }

    #[export]
    fn _process(&self, owner: &Node, _delta: f64) {
        for run in self.run_buffer.borrow_mut().iter_mut() {
            process_cmd(owner, &mut self.result_buffer.borrow_mut(), run);
        }
    }

    #[export]
    pub(crate) fn on_cmd_entered(&self, owner: &Node, text: String) -> Option<()> {
        godot_print!("on_cmd_entered: {text}!");
        owner.emit_signal("on_cmd_entered", &[Variant::new(text.clone())]);

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
        let first = run.cmds.first()?;
        owner.emit_signal("on_cmd_parsed", &[Variant::new(first)]);
        self.run_buffer.borrow_mut().push(run);
        Some(())
    }

    pub fn receive_command_result(&self, result: CommandResult) -> Option<()> {
        godot_print!("receive_command_result: {:?}!", result);
        self.result_buffer.borrow_mut().insert(result.id, result);
        Some(())
    }
}

fn process_cmd(owner: &Node, result_buffer: &mut ResultBuffer, run: &mut CommandRun) -> Option<()> {
    let done: Vec<&CommandInput> = run
        .cmds
        .iter()
        .take_while(|&cmd| {
            if let Some(result) = result_buffer.remove(&cmd.id) {
                godot_print!("on_cmd_result: {:?}", result);
                owner.emit_signal("on_cmd_result", &[Variant::new(result)]);
                run.active_id += 1;
                return true;
            }
            false
        })
        .collect();

    if done.len() > 0 {
        let cmd = run.cmds.first()?.clone();
        owner.emit_signal("on_cmd_parsed", &[Variant::new(cmd)]);
    }
    Some(())
}
