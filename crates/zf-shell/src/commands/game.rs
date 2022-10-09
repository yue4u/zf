use crate::imports;
use nu_protocol::{engine::Command, IntoPipelineData, ShellError, Signature, Value};
use zf_bridge::{GameCommand, ZFCommandArgs};

#[derive(Clone)]
pub(crate) struct Game;

impl Command for Game {
    fn name(&self) -> &str {
        "game"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("game")
    }

    fn usage(&self) -> &str {
        "game"
    }

    fn run(
        &self,
        _engine_state: &nu_protocol::engine::EngineState,
        _stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::ast::Call,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, ShellError> {
        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}

#[derive(Clone)]
pub(crate) struct GameStart;

impl Command for GameStart {
    fn name(&self) -> &str {
        "game start"
    }

    fn signature(&self) -> Signature {
        Signature::build("game start")
    }

    fn usage(&self) -> &str {
        "Start game"
    }

    fn run(
        &self,
        _engine_state: &nu_protocol::engine::EngineState,
        _stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::ast::Call,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, ShellError> {
        let args = ZFCommandArgs::Game(GameCommand::Start);
        imports::zf_call(args);
        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}

#[derive(Clone)]
pub(crate) struct GameMenu;

impl Command for GameMenu {
    fn name(&self) -> &str {
        "game menu"
    }

    fn signature(&self) -> Signature {
        Signature::build("game menu")
    }

    fn usage(&self) -> &str {
        "Goto game menu"
    }

    fn run(
        &self,
        _engine_state: &nu_protocol::engine::EngineState,
        _stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::ast::Call,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, ShellError> {
        let args = ZFCommandArgs::Game(GameCommand::Menu);
        imports::zf_call(args);

        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}

#[derive(Clone)]
pub(crate) struct GameEnd;

impl Command for GameEnd {
    fn name(&self) -> &str {
        "game end"
    }

    fn signature(&self) -> Signature {
        Signature::build("game end")
    }

    fn usage(&self) -> &str {
        "End game"
    }

    fn run(
        &self,
        _engine_state: &nu_protocol::engine::EngineState,
        _stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::ast::Call,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, ShellError> {
        let args = ZFCommandArgs::Game(GameCommand::End);
        imports::zf_call(args);

        Ok(Value::Nothing { span: call.head }.into_pipeline_data())
    }
}
