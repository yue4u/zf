use crate::ui::CommandPalette;
use crate::units::Player;
use crate::vm_connector::VMHost;

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
    VMHost => space::VMHOST,
    Player => space::T_MJOLNIR,
    CommandPalette => space::LINE_EDIT
);
