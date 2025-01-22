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
