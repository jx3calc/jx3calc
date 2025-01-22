mod attribute_type;
mod normal;

pub use attribute_type::*;
pub use normal::*;

use strum_macros::{Display, EnumCount, EnumIter};

#[allow(non_camel_case_types)]
#[derive(EnumCount, EnumIter, Display)]
pub enum FuncName {
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
        pub enum $name {
            $($variant = $value),*
        }
    };
    ($name:ident { $($variant:ident),* $(,)? }) => {
        #[allow(non_camel_case_types)]
        #[derive(Display, EnumIter)]
        #[repr(u8)]
        pub enum $name {
            $($variant),*
        }
    };
}
pub use enumxlua;
