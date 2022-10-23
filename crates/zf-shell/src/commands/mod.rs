mod engine;
mod fire;
mod game;
mod hi;
mod mission;
mod mystery;
mod radar;
mod ui;
mod zf_call;
mod task;

pub(crate) use engine::*;
pub(crate) use fire::*;
pub(crate) use game::*;
pub(crate) use hi::*;
pub(crate) use mission::*;
pub(crate) use mystery::*;
pub(crate) use radar::*;
pub(crate) use ui::*;
pub(crate) use task::*;

use nu_engine::CallExt;

use nu_protocol::{
    ast::Call,
    engine::{EngineState, Stack},
    FromValue, ShellError,
};

fn expect_flag<T>(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    name: &str,
) -> Result<T, ShellError>
where
    T: FromValue,
{
    match call.get_flag::<T>(engine_state, stack, name)? {
        Some(val) => Ok(val),
        _ => Err(ShellError::MissingParameter(
            format!("flag {} not exist", name),
            call.head,
        )),
    }
}
