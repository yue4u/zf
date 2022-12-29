use crate::{entities::LEVELS, managers::VMManager};

use self::path::LevelName;

pub mod groups;
pub mod path;

pub trait HasPath {
    fn path() -> &'static str;
}

#[macro_export]
macro_rules! bind_path {
    (
        $(
            $x:ty => $p1:ident::$p2:ident
        ),*
    ) => {
        $(
            impl HasPath for $x {
                fn path() -> &'static str {
                    crate::refs::path::$p1::$p2
                }
            }
        )*
    };
}

bind_path!(
    VMManager => auto_load::VM
);

pub fn next_level(current: String) -> Option<LevelName> {
    let current_idx = LEVELS.iter().position(|l| l.as_str() == &current)?;
    let next = LEVELS.get(current_idx + 1)?;
    Some(next.clone())
}