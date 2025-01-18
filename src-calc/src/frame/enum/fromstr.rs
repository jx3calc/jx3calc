pub mod skill;
pub mod skillevent;

use strum_macros::{Display, EnumString};

#[macro_export]
macro_rules! common_enum {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        #[allow(non_camel_case_types)]
        #[derive(EnumString, Display, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum $name {
            $($variant),*
        }
    };
}
pub use common_enum;

common_enum! { Attrib {
    atActiveThreatCoefficient,
    atAddExpPercent,
    atAddReputationPercent,
    atAddTransparencyValue,
    atAgilityBasePercentAdd,
    atAllDamageAddPercent,
    atAllMagicDamageAddPercent,
    atAllPhysicsDamageAddPercent,
    atAllShieldIgnorePercent,
    atAllTypeCriticalStrike,
    atBasePotentialAdd,
    atBeImmunisedStealthEnable,
    atBeTherapyCoefficient,
    atCallBuff,
    atCallLunarDamage,
    atCallSolarDamage,
    atCastSkillTargetDst,
    atDamageToLifeForSelf,
    atDstNpcDamageCoefficient,
    atExecuteScript,
    atFormationEffect,
    atGlobalResistPercent,
    atHalt,
    atHasteBase,
    atImmuneSkillMove,
    atImmunity,
    atKnockedBackRate,
    atKnockedDownRate,
    atKnockedOffRate,
    atLunarAttackPowerPercent,
    atLunarCriticalDamagePowerBaseKiloNumRate,
    atLunarCriticalStrikeBaseRate,
    atLunarDamageCoefficient,
    atLunarMagicShieldPercent,
    atLunarOvercomePercent,
    atMagicAttackPowerBase,
    atMagicAttackPowerPercent,
    atMagicCriticalDamagePowerBaseKiloNumRate,
    atMagicOvercome,
    atMagicShield,
    atMaxSkillRadiusPercent,
    atMoveSpeedPercent,
    atNeutralAttackPowerPercent,
    atNeutralCriticalStrikeBaseRate,
    atNeutralDamageCoefficient,
    atNeutralMagicShieldPercent,
    atNeutralOvercomePercent,
    atNoLimitChangeSkillIcon,
    atPhysicsAttackPowerPercent,
    atPhysicsCriticalDamagePowerBaseKiloNumRate,
    atPhysicsCriticalStrikeBaseRate,
    atPhysicsDamageCoefficient,
    atPhysicsOvercomeBase,
    atPhysicsOvercomePercent,
    atPoisonAttackPowerPercent,
    atPoisonCriticalStrikeBaseRate,
    atPoisonDamageCoefficient,
    atPoisonMagicShieldPercent,
    atPoisonOvercomePercent,
    atRepulsedRate,
    atSetSelectableType,
    atSetTalentRecipe,
    atSkillEventHandler,
    atSolarAttackPowerPercent,
    atSolarCriticalDamagePowerBaseKiloNumRate,
    atSolarCriticalStrikeBaseRate,
    atSolarDamageCoefficient,
    atSolarMagicShieldPercent,
    atSolarOvercomePercent,
    atSpiritBasePercentAdd,
    atSpunkBasePercentAdd,
    atStrengthBasePercentAdd,
    atSpunkBase,
    atStealth,
    atStrainBase,
    atStrainPercent,
    atStrainRate,
    atSurplusValueBase,
    atTherapyPowerBase,
    atToughnessBaseRate,
    atTransferDamage,
}}
