mod attrib;
pub(crate) mod skill;
pub(crate) mod skillevent;

pub(crate) use attrib::*;

use strum_macros::EnumString;

macro_rules! enumfromstr {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, EnumString, Eq, PartialEq)]
        #[repr(u8)]
        pub(crate) enum $name {
            $($variant),*
        }
    };
}
use enumfromstr;
