use super::*;

enumfromstr! { KindType {
    None,
    Physics,
    SolarMagic,
    LunarMagic,
    NeutralMagic,
    Poison,
    Leap,
    Adaptive,
}}

impl Default for KindType {
    fn default() -> Self {
        KindType::None
    }
}

enumfromstr! { CastMode {
    CasterArea,
    CasterAreaOfAttention,
    CasterAreaOfDepth,
    CasterConvexHullArea,
    CasterSingle,
    CasterSpreadCircle,
    Item,
    PartyArea,
    Point,
    PointArea,
    PointAreaFindFirst,
    PointAreaOfCasterTeam,
    PointRectangle,
    Rectangle,
    RectangleOfDepth,
    Sector,
    SectorOfAttention,
    SectorOfDepth,
    TargetAngleRectangle,
    TargetAngleSector,
    TargetArea,
    TargetChain,
    TargetHoodle,
    TargetLeader,
    TargetRay,
    TargetSingle,
    TargetTeamArea,
}}

impl Default for CastMode {
    fn default() -> Self {
        CastMode::TargetSingle
    }
}
