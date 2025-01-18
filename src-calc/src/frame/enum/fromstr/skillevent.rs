use super::*;

common_enum! { Type {
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

common_enum! { Role {
    EventCaster,
    EventTarget,
}}
