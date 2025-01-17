use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[allow(non_camel_case_types)]
#[derive(EnumIter, Display)]
pub enum Buff {
    ID,
    Level,
    IsStackable,
    MaxStackNum,
    Count,
    Interval,
    Hide,
    Exclude,
    ScriptFile,
    CanCancel,
    MinInterval,
    MaxInterval,
}

impl Buff {
    pub const BEGIN: usize = 15;
    pub const ACTIVE: usize = 2;
    pub const END_TIME: usize = 2;
    pub fn to_fields() -> Vec<String> {
        let mut res: Vec<String> = Buff::iter().map(|x| x.to_string()).collect();
        for i in 0..Buff::BEGIN {
            res.push(format!("BeginAttrib{}", i + 1));
            res.push(format!("BeginValue{}A", i + 1));
            res.push(format!("BeginValue{}B", i + 1));
        }
        for i in 0..Buff::ACTIVE {
            res.push(format!("ActiveAttrib{}", i + 1));
            res.push(format!("ActiveValue{}A", i + 1));
            res.push(format!("ActiveValue{}B", i + 1));
        }
        for i in 0..Buff::END_TIME {
            res.push(format!("EndTimeAttrib{}", i + 1));
            res.push(format!("EndTimeValue{}A", i + 1));
            res.push(format!("EndTimeValue{}B", i + 1));
        }
        res
    }
}

#[allow(non_camel_case_types)]
#[derive(EnumIter, Display)]
pub enum BuffUI {
    BuffID,
    Level,
    Name,
}

impl BuffUI {
    pub fn to_fields() -> Vec<String> {
        BuffUI::iter().map(|x| x.to_string()).collect()
    }
}
