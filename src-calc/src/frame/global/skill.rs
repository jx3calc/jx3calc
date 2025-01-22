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
pub struct Skill {
    id: i32,
    level: i64,
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
    dw_level_up_exp: i64,      // 升级经验
    n_exp_add_odds: i64,       // 技能熟练度增长概率
    n_player_level_limit: i64, // 角色可以学会该技能所必须达到的最低等级

    // 技能仇恨
    n_base_threat: i64,

    // 技能消耗
    n_cost_life: i64,              // 技能消耗生命值
    n_cost_mana: i64,              // 技能消耗的内力
    n_cost_stamina: i64,           // 技能消耗的体力
    n_cost_item_type: i64,         // 技能消耗的物品类型
    n_cost_item_index: i64,        // 技能消耗的物品索引ID
    n_cost_mana_base_percent: i64, // 技能消耗的内力百分比
    n_cost_sprint_power: i64,      // 技能消耗气力值

    // 聚气相关
    b_is_accumulate: bool, // 技能是否需要聚气

    // 链状技能相关
    n_chain_branch: i64, // 链状技能分支数
    n_chain_depth: i64,  // 链状技能层数

    // 施放距离
    n_min_radius: i64, // 技能施放的最小距离
    n_max_radius: i64, // 技能施放的最大距离

    // 作用范围
    n_protect_radius: i64, // 环形和矩形AOE的保护距离，范围内不受伤害
    n_height: i64, // AOE的高度，全高，圆柱体AOE中不填为2倍的nAreaRadius，矩形AOE中不填为nAreaRadius
    n_rect_width: i64, // 矩形AOE的宽度，全宽，不填为nAreaRadius
    n_angle_range: i64, // 攻击范围的扇形角度范围
    b_full_angle_in_air: bool,
    n_area_radius: i64,           // 技能作用半径
    n_target_count_limit: i64,    // 技能作用目标数量限制,(小于0代表目标数量不限制)
    b_ignore_prepare_state: bool, // 技能是否可在吟唱中施放，吟唱、通道、蓄力技不能填true

    // 时间相关
    n_prepare_frames: i64,   // 吟唱帧数
    n_channel_interval: i64, // 通道技间隔时间
    n_channel_frame: i64,    // 通道技持续时间，单位帧数
    n_bullet_velocity: i64,  // 子弹速度，单位 点/帧

    // 阵法相关
    b_is_sun_moon_power: bool,         // 技能是否需要日月豆
    sun_subsection_skill_id: i32,      // 日豆技能ID
    sun_subsection_skill_level: i32,   // 日豆技能等级
    moon_subsection_skill_id: i32,     // 月豆技能ID
    moon_subsection_skill_level: i32,  // 月豆技能等级
    b_is_formation_skill: bool,        // 是否阵眼技能
    n_formation_range: i64,            // 结阵的范围
    n_least_formation_population: i64, // 结阵的范围的最少队员数（包括队长）

    // 目标血量需求
    n_target_life_percent_min: i64, // 血量最小值>=
    n_target_life_percent_max: i64, // 血量最大值<=

    // 自身血量需求
    n_self_life_percent_min: i64, // 血量最小值>=
    n_self_life_percent_max: i64, // 血量最大值<=

    // 打退打断落马相关
    n_beat_back_rate: i64,   // 技能被打退的概率,默认1024
    n_broken_rate: i64,      // 技能被打断的概率,默认1024
    n_break_rate: i64,       // 打断目标施法的概率,基数1024
    n_dismounting_rate: i64, // 将目标击落下马几率,基数1024，默认0

    // 武器伤害相关
    n_weapon_damage_percent: i64, // 武器伤害百分比,对外功伤害有用。填0表示此次外功攻击不计算武器伤害,1024为100%
}

/* sub structs */

struct UI(String);

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

pub fn get(id: i32, level: i32) -> Option<&'static Skill> {
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

            level: 0,
            name: None,
            attributes: Vec::new(),
            check_buffs: Vec::new(),
            check_self_learnt_skills: Vec::new(),
            bind_buff: [const { None }; 4],
            cooldown: Cooldown {
                public: None,
                normal: [None; 3],
                normal_add: [0; 3],
                check: [None; 3],
            },
            delay_sub_skills: Vec::new(),

            dw_level_up_exp: 0,
            n_exp_add_odds: 0,
            n_player_level_limit: 0,
            n_base_threat: 0,
            n_cost_life: 0,
            n_cost_mana: 0,
            n_cost_stamina: 0,
            n_cost_item_type: 0,
            n_cost_item_index: 0,
            n_cost_mana_base_percent: 0,
            n_cost_sprint_power: 0,
            b_is_accumulate: false,
            n_chain_branch: 0,
            n_chain_depth: 0,
            n_min_radius: 0,
            n_max_radius: 0,
            n_protect_radius: 0,
            n_height: 0,
            n_rect_width: 0,
            n_angle_range: 0,
            b_full_angle_in_air: false,
            n_area_radius: 0,
            n_target_count_limit: 0,
            b_ignore_prepare_state: false,
            n_prepare_frames: 0,
            n_channel_interval: 0,
            n_channel_frame: 0,
            n_bullet_velocity: 0,
            b_is_sun_moon_power: false,
            sun_subsection_skill_id: 0,
            sun_subsection_skill_level: 0,
            moon_subsection_skill_id: 0,
            moon_subsection_skill_level: 0,
            b_is_formation_skill: false,
            n_formation_range: 0,
            n_least_formation_population: 0,
            n_target_life_percent_min: 0,
            n_target_life_percent_max: 0,
            n_self_life_percent_min: 0,
            n_self_life_percent_max: 0,
            n_beat_back_rate: 0,
            n_broken_rate: 0,
            n_break_rate: 0,
            n_dismounting_rate: 0,
            n_weapon_damage_percent: 0,
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
            skill.n_weapon_damage_percent = 1024;
        }

        // 处理 level
        let count = Field::iter().count();
        let level: i32 = data[count].parse().ok()?; // Will not fail
        skill.level = if level > 0 { level } else { skill.max_level } as i64;

        // 处理 ScriptFile
        let scriptfile = format!("scripts/skill/{}", &data[Field::ScriptFile as usize]);
        match skill.get_skill_level_data(&scriptfile) {
            Ok(_) => {}
            Err(e) => {
                error!(
                    "{}, {} GetSkillLevelData failed:\n{}",
                    skill.id, skill.level, e
                );
                return None;
            }
        }

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

impl Skill {
    fn get_skill_level_data(&mut self, scriptfile: &str) -> mlua::Result<()> {
        let lua_func = lua::get_func(scriptfile, FuncName::GetSkillLevelData)?.ok_or(
            mlua::Error::external(format!(
                "[global::skill] No GetSkillLevelData function in {}",
                scriptfile
            )),
        )?;

        let skill = lua::create_table()?;
        let meta = lua::create_table()?;
        let r0 = self as *mut Skill;
        let r1 = self as *const Skill;

        lua::scope(|scope| {
            let add_attr = scope.create_function_mut(|_, (a, b, c, d)| -> mlua::Result<()> {
                unsafe { Skill::add_attribute(r0, a, b, c, d) }
            })?;
            skill.set("AddAttribute", add_attr)?;

            type GT = (mlua::Table, String);
            let getter = scope.create_function(|_, (_, k): GT| -> mlua::Result<mlua::Value> {
                unsafe { Skill::getter(r1, k) }
            })?;
            type ST = (mlua::Table, String, mlua::Value);
            let setter = scope.create_function_mut(|_, (_, k, v): ST| -> mlua::Result<()> {
                unsafe { Skill::setter(r0, k, v) }
            })?;
            meta.set("__index", getter).unwrap();
            meta.set("__newindex", setter).unwrap();
            skill.set_metatable(Some(meta));

            lua_func.call::<mlua::Value>(skill)?;
            Ok(())
        })?;
        Ok(())
    }

    unsafe fn add_attribute(
        skill: *mut Skill,
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
        (*skill).attributes.push(Attribute {
            mode,
            r#type,
            param1,
            param2,
        });
        Ok(())
    }

    unsafe fn getter(skill: *const Skill, k: String) -> mlua::Result<mlua::Value> {
        println!("getter: {}", k);
        let s = &*skill;
        let res = match k.as_str() {
            "dwLevel" => mlua::Value::Integer(s.level),
            "dwLevelUpExp" => mlua::Value::Integer(s.dw_level_up_exp),
            "nExpAddOdds" => mlua::Value::Integer(s.n_exp_add_odds),
            "nPlayerLevelLimit" => mlua::Value::Integer(s.n_player_level_limit),
            "nBaseThreat" => mlua::Value::Integer(s.n_base_threat),
            "nCostLife" => mlua::Value::Integer(s.n_cost_life),
            "nCostMana" => mlua::Value::Integer(s.n_cost_mana),
            "nCostStamina" => mlua::Value::Integer(s.n_cost_stamina),
            "nCostItemType" => mlua::Value::Integer(s.n_cost_item_type),
            "nCostItemIndex" => mlua::Value::Integer(s.n_cost_item_index),
            "nCostManaBasePercent" => mlua::Value::Integer(s.n_cost_mana_base_percent),
            "nCostSprintPower" => mlua::Value::Integer(s.n_cost_sprint_power),
            "bIsAccumulate" => mlua::Value::Boolean(s.b_is_accumulate),
            "nChainBranch" => mlua::Value::Integer(s.n_chain_branch),
            "nChainDepth" => mlua::Value::Integer(s.n_chain_depth),
            "nMinRadius" => mlua::Value::Integer(s.n_min_radius),
            "nMaxRadius" => mlua::Value::Integer(s.n_max_radius),
            "nProtectRadius" => mlua::Value::Integer(s.n_protect_radius),
            "nHeight" => mlua::Value::Integer(s.n_height),
            "nRectWidth" => mlua::Value::Integer(s.n_rect_width),
            "nAngleRange" => mlua::Value::Integer(s.n_angle_range),
            "bFullAngleInAir" => mlua::Value::Boolean(s.b_full_angle_in_air),
            "nAreaRadius" => mlua::Value::Integer(s.n_area_radius),
            "nTargetCountLimit" => mlua::Value::Integer(s.n_target_count_limit),
            "bIgnorePrepareState" => mlua::Value::Boolean(s.b_ignore_prepare_state),
            "nPrepareFrames" => mlua::Value::Integer(s.n_prepare_frames),
            "nChannelInterval" => mlua::Value::Integer(s.n_channel_interval),
            "nChannelFrame" => mlua::Value::Integer(s.n_channel_frame),
            "nBulletVelocity" => mlua::Value::Integer(s.n_bullet_velocity),
            "bIsSunMoonPower" => mlua::Value::Boolean(s.b_is_sun_moon_power),
            "bIsFormationSkill" => mlua::Value::Boolean(s.b_is_formation_skill),
            "nFormationRange" => mlua::Value::Integer(s.n_formation_range),
            "nLeastFormationPopulation" => mlua::Value::Integer(s.n_least_formation_population),
            "nTargetLifePercentMin" => mlua::Value::Integer(s.n_target_life_percent_min),
            "nTargetLifePercentMax" => mlua::Value::Integer(s.n_target_life_percent_max),
            "nSelfLifePercentMin" => mlua::Value::Integer(s.n_self_life_percent_min),
            "nSelfLifePercentMax" => mlua::Value::Integer(s.n_self_life_percent_max),
            "nBeatBackRate" => mlua::Value::Integer(s.n_beat_back_rate),
            "nBrokenRate" => mlua::Value::Integer(s.n_broken_rate),
            "nBreakRate" => mlua::Value::Integer(s.n_break_rate),
            "nDismountingRate" => mlua::Value::Integer(s.n_dismounting_rate),
            "nWeaponDamagePercent" => mlua::Value::Integer(s.n_weapon_damage_percent),
            _ => mlua::Value::Nil,
        };
        Ok(res)
    }

    unsafe fn setter(skill: *mut Skill, k: String, v: mlua::Value) -> mlua::Result<()> {
        println!("setter: {} {:?}", k, v);
        fn iv(t: &mut i64, v: mlua::Value) {
            match v {
                mlua::Value::Integer(v) => *t = v,
                mlua::Value::Number(v) => *t = v as i64,
                _ => (),
            }
        }
        fn bv(t: &mut bool, v: mlua::Value) {
            match v {
                mlua::Value::Boolean(v) => *t = v,
                _ => (),
            }
        }
        let s = &mut *skill;
        match k.as_str() {
            "dwLevel" => iv(&mut s.level, v),
            "dwLevelUpExp" => iv(&mut s.dw_level_up_exp, v),
            "nExpAddOdds" => iv(&mut s.n_exp_add_odds, v),
            "nPlayerLevelLimit" => iv(&mut s.n_player_level_limit, v),
            "nBaseThreat" => iv(&mut s.n_base_threat, v),
            "nCostLife" => iv(&mut s.n_cost_life, v),
            "nCostMana" => iv(&mut s.n_cost_mana, v),
            "nCostStamina" => iv(&mut s.n_cost_stamina, v),
            "nCostItemType" => iv(&mut s.n_cost_item_type, v),
            "nCostItemIndex" => iv(&mut s.n_cost_item_index, v),
            "nCostManaBasePercent" => iv(&mut s.n_cost_mana_base_percent, v),
            "nCostSprintPower" => iv(&mut s.n_cost_sprint_power, v),
            "bIsAccumulate" => bv(&mut s.b_is_accumulate, v),
            "nChainBranch" => iv(&mut s.n_chain_branch, v),
            "nChainDepth" => iv(&mut s.n_chain_depth, v),
            "nMinRadius" => iv(&mut s.n_min_radius, v),
            "nMaxRadius" => iv(&mut s.n_max_radius, v),
            "nProtectRadius" => iv(&mut s.n_protect_radius, v),
            "nHeight" => iv(&mut s.n_height, v),
            "nRectWidth" => iv(&mut s.n_rect_width, v),
            "nAngleRange" => iv(&mut s.n_angle_range, v),
            "bFullAngleInAir" => bv(&mut s.b_full_angle_in_air, v),
            "nAreaRadius" => iv(&mut s.n_area_radius, v),
            "nTargetCountLimit" => iv(&mut s.n_target_count_limit, v),
            "bIgnorePrepareState" => bv(&mut s.b_ignore_prepare_state, v),
            "nPrepareFrames" => iv(&mut s.n_prepare_frames, v),
            "nChannelInterval" => iv(&mut s.n_channel_interval, v),
            "nChannelFrame" => iv(&mut s.n_channel_frame, v),
            "nBulletVelocity" => iv(&mut s.n_bullet_velocity, v),
            "bIsSunMoonPower" => bv(&mut s.b_is_sun_moon_power, v),
            "bIsFormationSkill" => bv(&mut s.b_is_formation_skill, v),
            "nFormationRange" => iv(&mut s.n_formation_range, v),
            "nLeastFormationPopulation" => iv(&mut s.n_least_formation_population, v),
            "nTargetLifePercentMin" => iv(&mut s.n_target_life_percent_min, v),
            "nTargetLifePercentMax" => iv(&mut s.n_target_life_percent_max, v),
            "nSelfLifePercentMin" => iv(&mut s.n_self_life_percent_min, v),
            "nSelfLifePercentMax" => iv(&mut s.n_self_life_percent_max, v),
            "nBeatBackRate" => iv(&mut s.n_beat_back_rate, v),
            "nBrokenRate" => iv(&mut s.n_broken_rate, v),
            "nBreakRate" => iv(&mut s.n_break_rate, v),
            "nDismountingRate" => iv(&mut s.n_dismounting_rate, v),
            "nWeaponDamagePercent" => iv(&mut s.n_weapon_damage_percent, v),
            _ => (),
        };
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
        assert_eq!(value.level, 1);
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
        assert_eq!(value.dw_level_up_exp, 110);
        assert_eq!(value.n_cost_mana, 3);
        assert_eq!(value.n_min_radius, 0);
        assert_eq!(value.n_max_radius, 2048);
        assert_eq!(value.n_area_radius, 3200);
        assert_eq!(value.n_angle_range, 256);
        assert_eq!(value.n_prepare_frames, 0);
        assert_eq!(value.n_bullet_velocity, 0);
        assert_eq!(value.n_break_rate, 307);
        assert_eq!(value.n_channel_frame, 960);
        assert_eq!(value.n_channel_interval, 32);
    }
}
