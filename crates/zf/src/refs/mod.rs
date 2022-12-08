use crate::managers::VMManager;
// use crate::ui::CommandPalette;

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
    VMManager => base::VM
    // CommandPalette => base::LINE_EDIT
);
