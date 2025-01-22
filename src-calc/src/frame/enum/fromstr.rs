mod attrib;
pub mod skill;
pub mod skillevent;

pub use attrib::*;

use strum_macros::EnumString;

#[macro_export]
macro_rules! enumfromstr {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, EnumString, Eq, PartialEq)]
        #[repr(u8)]
        pub enum $name {
            $($variant),*
        }
    };
}
pub use enumfromstr;
