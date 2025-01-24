use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[allow(non_camel_case_types)]
#[derive(Display, EnumIter)]
pub(crate) enum Buff {
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
    pub(crate) const BEGIN: usize = 15;
    pub(crate) const ACTIVE: usize = 2;
    pub(crate) const END_TIME: usize = 2;
    pub(crate) fn to_fields() -> Vec<String> {
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

macro_rules! enumtostr {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        #[allow(non_camel_case_types)]
        #[derive(Display, EnumIter)]
        pub(crate) enum $name {
            $($variant),*
        }

        impl $name {
            pub(crate) fn to_fields() -> Vec<String> {
                $name::iter().map(|x| x.to_string()).collect()
            }
        }
    };
}

enumtostr! { Cooldown {
    ID,
    Duration,
    MinDuration,
    MaxDuration,
    MaxCount,
}}

enumtostr! { CustomTrinket {
    ID,
    SkillID,
    SkillLevel,
    CoolDownID,
}}

enumtostr! { SkillEvent {
    ID,
    EventType,
    Odds,
    SkillID,
    SkillLevel,
    SkillCaster,
    SkillTarget,
    EventMask1,
    EventMask2,
    EventSkillID,
}}

enumtostr! { SkillRecipe {
    RecipeID,
    RecipeLevel,
    SkillRecipeType,
    SkillID,
    CoolDownAdd1,
    CoolDownAdd2,
    CoolDownAdd3,
    DamageAddPercent,
    ScriptFile,
}}

enumtostr! { Skill {
    SkillID,
    MaxLevel,
    KindType,
    CastMode,
    MountRequestType,
    MountRequestDetail,
    IsPassiveSkill,
    HasCriticalStrike,
    SkillEventMask1,
    SkillEventMask2,
    NeedOutOfFight,
    TargetTypePlayer,
    TargetTypeNpc,
    TargetRelationNone,
    TargetRelationSelf,
    TargetRelationEnemy,
    RecipeType,
    IsFrost,
    WeaponRequest,
    ScriptFile,
}}
