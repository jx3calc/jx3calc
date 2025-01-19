use crate::frame::r#enum::{
    fromstr::skill::{CastMode, KindType},
    // tostr::Skill as Field,
};
// use pak::{tab_get, tab_init};

// use log::error;
// use once_cell::sync::Lazy;

/* structs */

/// Skill
struct Skill {
    id: i32,
    level: i32,

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
    dw_level_up_exp: i32,      // 升级经验
    n_exp_add_odds: i32,       // 技能熟练度增长概率
    n_player_level_limit: i32, // 角色可以学会该技能所必须达到的最低等级

    // 技能仇恨
    n_base_threat: i32,

    // 技能消耗
    n_cost_life: i32,              // 技能消耗生命值
    n_cost_mana: i32,              // 技能消耗的内力
    n_cost_stamina: i32,           // 技能消耗的体力
    n_cost_item_type: i32,         // 技能消耗的物品类型
    n_cost_item_index: i32,        // 技能消耗的物品索引ID
    n_cost_mana_base_percent: i32, // 技能消耗的内力百分比
    n_cost_sprint_power: i32,      // 技能消耗气力值

    // 聚气相关
    b_is_accumulate: bool, // 技能是否需要聚气

    // 链状技能相关
    n_chain_branch: i32, // 链状技能分支数
    n_chain_depth: i32,  // 链状技能层数

    // 施放距离
    n_min_radius: i32, // 技能施放的最小距离
    n_max_radius: i32, // 技能施放的最大距离

    // 作用范围
    n_protect_radius: i32, // 环形和矩形AOE的保护距离，范围内不受伤害
    n_height: i32, // AOE的高度，全高，圆柱体AOE中不填为2倍的nAreaRadius，矩形AOE中不填为nAreaRadius
    n_rect_width: i32, // 矩形AOE的宽度，全宽，不填为nAreaRadius
    n_angle_range: i32, // 攻击范围的扇形角度范围
    b_full_angle_in_air: bool,
    n_area_radius: i32,           // 技能作用半径
    n_target_count_limit: i32,    // 技能作用目标数量限制,(小于0代表目标数量不限制)
    b_ignore_prepare_state: bool, // 技能是否可在吟唱中施放，吟唱、通道、蓄力技不能填true

    // 时间相关
    n_prepare_frames: i32,   // 吟唱帧数
    n_channel_interval: i32, // 通道技间隔时间
    n_channel_frame: i32,    // 通道技持续时间，单位帧数
    n_bullet_velocity: i32,  // 子弹速度，单位 点/帧

    // 阵法相关
    b_is_sun_moon_power: bool,         // 技能是否需要日月豆
    sun_subsection_skill_id: i32,      // 日豆技能ID
    sun_subsection_skill_level: i32,   // 日豆技能等级
    moon_subsection_skill_id: i32,     // 月豆技能ID
    moon_subsection_skill_level: i32,  // 月豆技能等级
    b_is_formation_skill: bool,        // 是否阵眼技能
    n_formation_range: i32,            // 结阵的范围
    n_least_formation_population: i32, // 结阵的范围的最少队员数（包括队长）

    // 目标血量需求
    n_target_life_percent_min: i32, // 血量最小值>=
    n_target_life_percent_max: i32, // 血量最大值<=

    // 自身血量需求
    n_self_life_percent_min: i32, // 血量最小值>=
    n_self_life_percent_max: i32, // 血量最大值<=

    // 打退打断落马相关
    n_beat_back_rate: i32,   // 技能被打退的概率,默认1024
    n_broken_rate: i32,      // 技能被打断的概率,默认1024
    n_break_rate: i32,       // 打断目标施法的概率,基数1024
    n_dismounting_rate: i32, // 将目标击落下马几率,基数1024，默认0

    // 武器伤害相关
    n_weapon_damage_percent: i32, // 武器伤害百分比,对外功伤害有用。填0表示此次外功攻击不计算武器伤害,1024为100%
}

/* sub structs */

#[derive(Debug, PartialEq, Eq)]
enum AttributeValue {
    Int(i32),
    String(String),
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
