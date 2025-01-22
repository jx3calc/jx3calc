use super::*;

enumfromstr! { Type {
    BeCast,
    BeCriticalStrike,
    BeDodge,
    BeHit,
    BeHitOTAction,
    BeKill,
    BeMiss,
    BeOverHeal,
    BlockLongRange,
    Cast,
    CriticalStrike,
    Dodge,
    Hit,
    HitOTAction,
    Kill,
    Miss,
    OverHeal,
    Parry,
    PreCast,
}}

enumfromstr! { Role {
    EventCaster,
    EventTarget,
}}
