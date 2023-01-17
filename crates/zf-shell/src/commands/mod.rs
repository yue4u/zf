mod audio;
mod clear;
mod credits;
mod engine;
mod fire;
mod fsays;
mod game;
mod hi;
mod hint;
mod level;
mod mystery;
mod radar;
mod random_pos;
mod shield;
mod special_thanks;
mod task;
mod term;
mod time;
mod tutorial;
mod ui;

pub(crate) use audio::*;
pub(crate) use clear::*;
pub(crate) use credits::*;
pub(crate) use engine::*;
pub(crate) use fire::*;
pub(crate) use fsays::*;
pub(crate) use game::*;
pub(crate) use hi::*;
pub(crate) use hint::*;
pub(crate) use level::*;
pub(crate) use mystery::*;
pub(crate) use radar::*;
pub(crate) use random_pos::*;
pub(crate) use shield::*;
pub(crate) use special_thanks::*;
pub(crate) use task::*;
pub(crate) use term::*;
pub(crate) use time::*;
pub(crate) use tutorial::*;
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
    match call.get_flag::<T>(engine_state, stack, name)? {
        Some(val) => Ok(val),
        _ => Err(ShellError::MissingParameter(
            format!("flag {} not exist", name),
            call.head,
        )),
    }
}
