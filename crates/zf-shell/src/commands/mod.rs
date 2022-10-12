mod engine;
mod fire;
mod game;
mod hi;
mod mission;
mod mystery;
mod radar;
mod ui;
mod zf_call;

pub(crate) use engine::*;
pub(crate) use fire::*;
pub(crate) use game::*;
pub(crate) use hi::*;
pub(crate) use mission::*;
pub(crate) use mystery::*;
pub(crate) use radar::*;
pub(crate) use ui::*;

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
    let flag: T = match call.get_flag(engine_state, stack, name)? {
        Some(val) => val,
        _ => {
            return Err(ShellError::MissingParameter(
                format!("flag {} not exist", name),
                call.head,
            ))
        }
    };
    Ok(flag)
}
