use crate::{entities::LEVELS, managers::VM};

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
    VM => auto_load::VM
);

pub fn next_level(current: String) -> Option<&'static LevelName> {
    let current_idx = LEVELS.iter().position(|l| l.as_str() == &current)?;
    LEVELS.get(current_idx + 1)
}
