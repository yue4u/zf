use crate::managers::VMManger;
use crate::ui::CommandPalette;
use crate::units::Player;

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
                    crate::path::path::$p1::$p2
                }
            }
        )*
    };
}

bind_path!(
    Player => space::T_MJOLNIR,
    VMManger => space::VM,
    CommandPalette => space::LINE_EDIT
);
