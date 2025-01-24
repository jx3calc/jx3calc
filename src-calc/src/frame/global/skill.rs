use crate::frame::{
    lua,
    r#enum::{
        fromstr::skill::{CastMode, KindType},
        tostr::Skill as Field,
        xlua::FuncName,
    },
};
use pak::{tab_get, tab_init};

use log::error;
use mlua;
use once_cell::sync::Lazy;
use strum::IntoEnumIterator;

/* static manager variable */
static SKILL: Lazy<super::Manager<(i32, i32), Skill>> = Lazy::new(super::Manager::new);
static SKILL_DATA: Lazy<super::Manager<i32, SkillData>> = Lazy::new(super::Manager::new);

/* struct */

struct SkillData(Vec<String>);

/// Skill
#[allow(non_snake_case)]
#[derive(Default)]
pub(super) struct Skill {
    id: i32,
    dwLevel: i32,
    name: Option<String>,

    max_level: i32,
    kind_type: KindType,
    cast_mode: CastMode,
    mount_request_type: i32,
    mount_request_detail: i32,
    is_passive_skill: bool,
    has_critical_strike: bool,
    skill_event_mask1: u32,
    skill_event_mask2: u32,
    need_out_of_fight: bool,
    target_type_player: bool,
    target_type_npc: bool,
    target_relation_none: bool,
    target_relation_self: bool,
    target_relation_enemy: bool,
    recipe_type: i32,
    is_frost: bool,

    // ---------- GetSkillLevelData 中初始化的字段 ----------
    attributes: Vec<Attribute>,
    check_buffs: Vec<CheckBuff>,
    check_self_learnt_skills: Vec<CheckSelfLearntSkill>,
    bind_buff: BindBuff,
    cooldown: Cooldown,
    delay_sub_skills: Vec<DelaySubSkill>,

    // 经验升级相关
    dwLevelUpExp: i32,      // 升级经验
    nExpAddOdds: i32,       // 技能熟练度增长概率
    nPlayerLevelLimit: i32, // 角色可以学会该技能所必须达到的最低等级

    // 技能仇恨
    nBaseThreat: i32,

    // 技能消耗
    nCostLife: i32,            // 技能消耗生命值
    nCostMana: i32,            // 技能消耗的内力
    nCostStamina: i32,         // 技能消耗的体力
    nCostItemType: i32,        // 技能消耗的物品类型
    nCostItemIndex: i32,       // 技能消耗的物品索引ID
    nCostManaBasePercent: i32, // 技能消耗的内力百分比
    nCostSprintPower: i32,     // 技能消耗气力值

    // 聚气相关
    bIsAccumulate: bool, // 技能是否需要聚气

    // 链状技能相关
    nChainBranch: i32, // 链状技能分支数
    nChainDepth: i32,  // 链状技能层数

    // 施放距离
    nMinRadius: i32, // 技能施放的最小距离
    nMaxRadius: i32, // 技能施放的最大距离

    // 作用范围
    nProtectRadius: i32, // 环形和矩形AOE的保护距离，范围内不受伤害
    nHeight: i32, // AOE的高度，全高，圆柱体AOE中不填为2倍的nAreaRadius，矩形AOE中不填为nAreaRadius
    nRectWidth: i32, // 矩形AOE的宽度，全宽，不填为nAreaRadius
    nAngleRange: i32, // 攻击范围的扇形角度范围
    bFullAngleInAir: bool,
    nAreaRadius: i32,          // 技能作用半径
    nTargetCountLimit: i32,    // 技能作用目标数量限制,(小于0代表目标数量不限制)
    bIgnorePrepareState: bool, // 技能是否可在吟唱中施放，吟唱、通道、蓄力技不能填true

    // 时间相关
    nPrepareFrames: i32,   // 吟唱帧数
    nChannelInterval: i32, // 通道技间隔时间
    nChannelFrame: i32,    // 通道技持续时间，单位帧数
    nBulletVelocity: i32,  // 子弹速度，单位 点/帧

    // 阵法相关
    bIsSunMoonPower: bool,            // 技能是否需要日月豆
    sun_subsection_skill_id: i32,     // 日豆技能ID
    sun_subsection_skill_level: i32,  // 日豆技能等级
    moon_subsection_skill_id: i32,    // 月豆技能ID
    moon_subsection_skill_level: i32, // 月豆技能等级
    bIsFormationSkill: bool,          // 是否阵眼技能
    nFormationRange: i32,             // 结阵的范围
    nLeastFormationPopulation: i32,   // 结阵的范围的最少队员数（包括队长）

    // 目标血量需求
    nTargetLifePercentMin: i32, // 血量最小值>=
    nTargetLifePercentMax: i32, // 血量最大值<=

    // 自身血量需求
    nSelfLifePercentMin: i32, // 血量最小值>=
    nSelfLifePercentMax: i32, // 血量最大值<=

    // 打退打断落马相关
    nBeatBackRate: i32,    // 技能被打退的概率,默认1024
    nBrokenRate: i32,      // 技能被打断的概率,默认1024
    nBreakRate: i32,       // 打断目标施法的概率,基数1024
    nDismountingRate: i32, // 将目标击落下马几率,基数1024，默认0

    // 武器伤害相关
    nWeaponDamagePercent: i32, // 武器伤害百分比,对外功伤害有用。填0表示此次外功攻击不计算武器伤害,1024为100%
}

/* sub structs */

struct UI(String);

#[allow(non_snake_case)]
struct SkillUserdata {
    skill: Skill,
    AddAttribute: mlua::Value,
    _marker: std::marker::PhantomPinned,
}

#[derive(Debug, PartialEq, Eq)]
enum AttributeValue {
    Int(i32),
    Str(String),
}

struct Attribute {
    mode: i32,
    r#type: i32,
    param1: AttributeValue,
    param2: AttributeValue,
}

enum CheckBuffType {
    CheckSelf,
    CheckDest,
    CheckSelfOwn,
    ChekDestOwn,
}

struct CheckBuff {
    r#type: CheckBuffType,
    buff_id: i32,
    stacknum: i32,
    stacknum_compare_flag: i32,
    level: i32,
    level_compare_flag: i32,
}

struct CheckSelfLearntSkill {
    id: i32,
    level: i32,
    level_compare_flag: i32,
}

struct BindBuffItem {
    id: i32,
    level: i32,
}

type BindBuff = [Option<BindBuffItem>; 4];

#[derive(Default)]
struct Cooldown {
    public: Option<i32>,
    normal: [Option<i32>; 3],
    normal_add: [i32; 3],
    check: [Option<i32>; 3],
}

struct DelaySubSkill {
    delay: i32,
    id: i32,
    level: i32,
}

/* impls */

pub(super) fn get(id: i32, level: i32) -> Option<&'static Skill> {
    SKILL.get(&(id, level))
}

impl super::SubTrait<i32> for SkillData {
    fn struct_name() -> &'static str {
        "SkillData"
    }
    fn tab_init() {
        let fields = Field::to_fields();
        let fields: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
        if !tab_init("settings/skill/skills.tab", &["SkillID"], &fields) {
            error!("Tab init failed");
        }
    }
    fn construct_from_tab(key: &i32) -> Option<Vec<String>> {
        match tab_get("skills.tab", &[&key.to_string()]) {
            Ok(res) => Some(res),
            Err(e) => {
                error!("{:?} not found:\n{}", key, e);
                None
            }
        }
    }
    fn parse_from_data(data: &[String]) -> Option<SkillData> {
        Some(SkillData(data.to_vec()))
    }
}

impl super::SubTrait<(i32, i32)> for Skill {
    fn struct_name() -> &'static str {
        "Skill"
    }
    fn tab_init() {
        SkillData::tab_init();
        UI::tab_init();
    }
    fn construct_from_tab(key: &(i32, i32)) -> Option<Vec<String>> {
        match SKILL_DATA.get(&(key.0)) {
            Some(res) => {
                let mut res = res.0.clone();
                res.push(key.1.to_string());
                match UI::construct_from_tab(key) {
                    Some(mut ui) => {
                        res.append(&mut ui);
                        Some(res)
                    }
                    None => Some(res),
                }
            }
            None => {
                error!("{:?} data get error.", key);
                None
            }
        }
    }
    fn parse_from_data(data: &[String]) -> Option<Skill> {
        // `.ok()` should be used when the field is never an empty string.
        // `.unwrap_or()` should be used if compatibility with empty strings is required.
        let mut skill = Skill {
            id: data[Field::SkillID as usize].parse().ok()?,
            max_level: data[Field::MaxLevel as usize].parse().unwrap_or(1),
            kind_type: data[Field::KindType as usize].parse().ok()?,
            cast_mode: data[Field::CastMode as usize].parse().ok()?,
            mount_request_type: data[Field::MountRequestType as usize].parse().unwrap_or(0),
            mount_request_detail: data[Field::MountRequestDetail as usize]
                .parse()
                .unwrap_or(0),
            is_passive_skill: data[Field::IsPassiveSkill as usize] == "1",
            has_critical_strike: data[Field::HasCriticalStrike as usize] == "1",
            skill_event_mask1: data[Field::SkillEventMask1 as usize].parse().unwrap_or(0),
            skill_event_mask2: data[Field::SkillEventMask2 as usize].parse().unwrap_or(0),
            need_out_of_fight: data[Field::NeedOutOfFight as usize] == "1",
            target_type_player: data[Field::TargetTypePlayer as usize] == "1",
            target_type_npc: data[Field::TargetTypeNpc as usize] == "1",
            target_relation_none: data[Field::TargetRelationNone as usize] == "1",
            target_relation_self: data[Field::TargetRelationSelf as usize] == "1",
            target_relation_enemy: data[Field::TargetRelationEnemy as usize] == "1",
            recipe_type: data[Field::RecipeType as usize].parse().unwrap_or(0),
            is_frost: data[Field::IsFrost as usize] == "1",
            ..Default::default()
        };

        // 处理默认武器伤害.
        // 目前推测: WeaponRequest 字段非 0 的技能默认拥有 1024 的武器伤害 (可以在后续 lua 的 getGetSkillLevelData 中被覆盖).
        // 注意: 拥有武器伤害不一定代表会造成武器伤害. 造成武器伤害与 AddAttribute 中的 CALL_PHYSICS_DAMAGE 有关.
        // 推测的依据:
        // 1. 部分技能并没有在 lua 中显式声明 nWeaponDamagePercent, 但是仍然可以造成武器伤害. (最简单的例子即为普通攻击)
        // 2. 部分不造成武器伤害的外功技能, 似乎都在 lua 中显式声明了其 nWeaponDamagePercent = 0. (例如, 丐帮的诸多需要武器施展的技能.)
        // 暂时按照该推测进行处理.
        let weapon_request = &data[Field::WeaponRequest as usize];
        if !weapon_request.is_empty() && weapon_request != "0" {
            skill.nWeaponDamagePercent = 1024;
        }

        // 处理 level
        let count = Field::iter().count();
        let level: i32 = data[count].parse().ok()?; // Will not fail
        skill.dwLevel = if level > 0 { level } else { skill.max_level };

        // 处理 ScriptFile
        let (id, level) = (skill.id, skill.dwLevel as i32);
        let scriptfile = format!("scripts/skill/{}", &data[Field::ScriptFile as usize]);
        let res = SkillUserdata::get_skill_level_data(skill, &scriptfile);
        let mut skill = match res {
            Ok(r) => r,
            Err(e) => {
                error!("{}, {} GetSkillLevelData failed:\n{}", id, level, e);
                return None;
            }
        };

        // 处理 UI
        if count + 1 < data.len() {
            skill.name = UI::parse_from_data(&data[count + 1..]).map(|ui| ui.0);
        }

        Some(skill)
    }
}

impl super::SubTrait<(i32, i32)> for UI {
    fn struct_name() -> &'static str {
        "SkillUI"
    }
    fn tab_init() {
        let fields = vec!["Name"];
        if !tab_init("ui/scheme/case/skill.txt", &["SkillID", "Level"], &fields) {
            error!("Tab init failed");
        }
    }
    fn construct_from_tab(key: &(i32, i32)) -> Option<Vec<String>> {
        match tab_get("skill.txt", &[&key.0.to_string(), &key.1.to_string()]) {
            Ok(res) => Some(res),
            Err(e) => {
                error!("{:?} not found:\n{}", key, e);
                None
            }
        }
    }
    fn parse_from_data(data: &[String]) -> Option<UI> {
        Some(UI(data[0].clone()))
    }
}

impl mlua::UserData for SkillUserdata {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        macro_rules! add_skill_fields {
            ($fields:expr, $($field:ident),*) => {
                $(
                    $fields.add_field_method_get(stringify!($field), |_, this| Ok(this.skill.$field));
                    $fields.add_field_method_set(stringify!($field), |_, this, value| {
                        this.skill.$field = value;
                        Ok(())
                    });
                )*
            };
        }
        add_skill_fields!(
            fields,
            dwLevel,
            dwLevelUpExp,
            nExpAddOdds,
            nPlayerLevelLimit,
            nBaseThreat,
            nCostLife,
            nCostMana,
            nCostStamina,
            nCostItemType,
            nCostItemIndex,
            nCostManaBasePercent,
            nCostSprintPower,
            bIsAccumulate,
            nChainBranch,
            nChainDepth,
            nMinRadius,
            nMaxRadius,
            nProtectRadius,
            nHeight,
            nRectWidth,
            nAngleRange,
            bFullAngleInAir,
            nAreaRadius,
            nTargetCountLimit,
            bIgnorePrepareState,
            nPrepareFrames,
            nChannelInterval,
            nChannelFrame,
            nBulletVelocity,
            bIsSunMoonPower,
            bIsFormationSkill,
            nFormationRange,
            nLeastFormationPopulation,
            nTargetLifePercentMin,
            nTargetLifePercentMax,
            nSelfLifePercentMin,
            nSelfLifePercentMax,
            nBeatBackRate,
            nBrokenRate,
            nBreakRate,
            nDismountingRate,
            nWeaponDamagePercent
        );
        macro_rules! add_functions {
            ($fields:expr, $($field:ident),*) => {
                $(
                    $fields.add_field_method_get(stringify!($field), |_, this| Ok(this.$field.clone()));
                )*
            };
        }
        add_functions!(fields, AddAttribute);
    }
}

impl SkillUserdata {
    fn get_skill_level_data(skill: Skill, scriptfile: &str) -> Result<Skill, mlua::Error> {
        let lua_func = lua::get_func(scriptfile, FuncName::GetSkillLevelData)?.ok_or(
            mlua::Error::external(format!(
                "[global::skill] No GetSkillLevelData function in {}",
                scriptfile
            )),
        )?;

        let mut ud = SkillUserdata {
            skill,
            AddAttribute: mlua::Value::Nil,
            _marker: std::marker::PhantomPinned,
        };
        let ptr = &mut ud.skill as *mut Skill;
        lua::scope(|scope| {
            let add_attribute = scope.create_function(|_, (a, b, c, d)| -> mlua::Result<()> {
                let this = unsafe { &mut *ptr };
                this.add_attribute(a, b, c, d)
            })?;
            ud.AddAttribute = mlua::Value::Function(add_attribute);

            let udr = scope.create_userdata_ref_mut(&mut ud)?;
            lua_func.call::<mlua::Value>(udr)?;
            Ok(())
        })?;

        Ok(ud.skill)
    }
}

impl Skill {
    fn add_attribute(
        &mut self,
        a: mlua::Value,
        b: mlua::Value,
        c: mlua::Value,
        d: mlua::Value,
    ) -> mlua::Result<()> {
        let mode = a.as_i32().ok_or(mlua::Error::runtime("mode is not i32"))?;
        let r#type = b.as_i32().ok_or(mlua::Error::runtime("type is not i32"))?;
        let param1 = match c {
            mlua::Value::Integer(v) => AttributeValue::Int(v as i32),
            mlua::Value::String(v) => AttributeValue::Str(v.clone().to_string_lossy()),
            _ => return Err(mlua::Error::runtime("param1 is not integer or string")),
        };
        let param2 = match d {
            mlua::Value::Integer(v) => AttributeValue::Int(v as i32),
            mlua::Value::String(v) => AttributeValue::Str(v.clone().to_string_lossy()),
            _ => return Err(mlua::Error::runtime("param2 is not integer or string")),
        };
        self.attributes.push(Attribute {
            mode,
            r#type,
            param1,
            param2,
        });
        Ok(())
    }
}

/* tests */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_pak() {
        env_logger::builder()
            .filter_level(log::LevelFilter::Trace)
            .init();
        let value = get(1, 1).unwrap();
        assert_eq!(value.id, 1);
        assert_eq!(value.dwLevel, 1);
        assert_eq!(value.name.as_ref().unwrap(), "测试技能");
        assert_eq!(value.max_level, 2);
        assert_eq!(value.kind_type, KindType::None);
        assert_eq!(value.cast_mode, CastMode::TargetSingle);
        assert_eq!(value.mount_request_type, 0);
        assert_eq!(value.mount_request_detail, 0);
        assert_eq!(value.is_passive_skill, false);
        assert_eq!(value.has_critical_strike, true);
        assert_eq!(value.skill_event_mask1, 0);
        assert_eq!(value.skill_event_mask2, 0);
        assert_eq!(value.need_out_of_fight, false);
        assert_eq!(value.target_type_player, true);
        assert_eq!(value.target_type_npc, true);
        assert_eq!(value.target_relation_none, true);
        assert_eq!(value.target_relation_self, true);
        assert_eq!(value.target_relation_enemy, true);
        assert_eq!(value.recipe_type, 0);
        assert_eq!(value.is_frost, false);
        assert_eq!(value.dwLevelUpExp, 110);
        assert_eq!(value.nCostMana, 3);
        assert_eq!(value.nMinRadius, 0);
        assert_eq!(value.nMaxRadius, 2048);
        assert_eq!(value.nAreaRadius, 3200);
        assert_eq!(value.nAngleRange, 256);
        assert_eq!(value.nPrepareFrames, 0);
        assert_eq!(value.nBulletVelocity, 0);
        assert_eq!(value.nBreakRate, 307);
        assert_eq!(value.nChannelFrame, 960);
        assert_eq!(value.nChannelInterval, 32);
    }
}
