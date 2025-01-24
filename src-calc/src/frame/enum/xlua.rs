mod attribute_type;
mod normal;

pub(crate) use attribute_type::*;
pub(crate) use normal::*;

use strum_macros::{Display, EnumCount, EnumIter};

#[allow(non_camel_case_types)]
#[derive(EnumCount, EnumIter, Display)]
pub(crate) enum FuncName {
    GetSkillLevelData,
    GetSkillRecipeData,
    Apply,
    UnApply,
    OnRemove,
    OnTimer,
    ApplySetup,
    UnApplySetup,
}

#[macro_export]
macro_rules! enumxlua {
    ($name:ident { $($variant:ident = $value:expr),* $(,)? }) => {
        #[allow(non_camel_case_types)]
        #[derive(Display, EnumIter)]
        #[repr(u8)]
        pub(crate) enum $name {
            $($variant = $value),*
        }
    };
    ($name:ident { $($variant:ident),* $(,)? }) => {
        #[allow(non_camel_case_types)]
        #[derive(Display, EnumIter)]
        #[repr(u8)]
        pub(crate) enum $name {
            $($variant),*
        }
    };
}
use enumxlua;
