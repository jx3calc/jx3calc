use crate::frame::r#enum::{
    fromstr::skillevent::{Role, Type},
    tostr::SkillEvent as Field,
};
use pak::{tab_get, tab_init};

use log::error;
use once_cell::sync::Lazy;

/* static manager variable */
static SKILL_EVENT: Lazy<super::Manager<i32, SkillEvent>> = Lazy::new(super::Manager::new);

/* struct */

/// SkillEvent
pub struct SkillEvent {
    id: i32,
    r#type: Type,
    odds: i32,
    skill_id: i32,
    skill_level: i32,
    skill_caster: Role,
    skill_target: Role,
    event_mask1: u32,
    event_mask2: u32,
    event_skill_id: i32,
    // event_skill_level: i32, // actually not used
}

/* impls */

pub fn get(id: i32) -> Option<&'static SkillEvent> {
    SKILL_EVENT.get(&id)
}

impl super::SubTrait<i32> for SkillEvent {
    fn struct_name() -> &'static str {
        "SkillEvent"
    }
    fn tab_init() {
        let fields = Field::to_fields();
        let fields: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
        if !tab_init("settings/skill/skillevent.tab", &["ID"], &fields) {
            error!("[global::skillevent] Tab init failed");
        }
    }
    fn construct_from_tab(key: &i32) -> Option<Vec<String>> {
        match tab_get("skillevent.tab", &[&key.to_string()]) {
            Ok(res) => Some(res),
            Err(e) => {
                error!("[global::skillevent] {:?} not found:\n{}", key, e);
                None
            }
        }
    }
    fn parse_from_data(data: &[String]) -> Option<SkillEvent> {
        Some(SkillEvent {
            // `.ok()` should be used when the field is never an empty string.
            // `.unwrap_or()` should be used if compatibility with empty strings is required.
            id: data[Field::ID as usize].parse().ok()?,
            r#type: data[Field::EventType as usize].parse().ok()?,
            odds: data[Field::Odds as usize].parse().ok()?,
            skill_id: data[Field::SkillID as usize].parse().ok()?,
            skill_level: data[Field::SkillLevel as usize].parse().ok()?,
            skill_caster: data[Field::SkillCaster as usize].parse().ok()?,
            skill_target: data[Field::SkillTarget as usize].parse().ok()?,
            event_mask1: data[Field::EventMask1 as usize].parse().ok()?,
            event_mask2: data[Field::EventMask2 as usize].parse().ok()?,
            event_skill_id: data[Field::EventSkillID as usize].parse().ok()?,
            // event_skill_level: res[Field::EventSkillLevel as usize].parse().ok()?,
        })
    }
}

/* tests */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_pak() {
        let value = get(0).unwrap();
        assert_eq!(value.id, 0);
        assert_eq!(value.r#type, Type::Hit);
        assert_eq!(value.odds, 1024);
        assert_eq!(value.skill_id, 19);
        assert_eq!(value.skill_level, 1);
        assert_eq!(value.skill_caster, Role::EventCaster);
        assert_eq!(value.skill_target, Role::EventCaster);
        assert_eq!(value.event_mask1, 1);
        assert_eq!(value.event_mask2, 0);
        assert_eq!(value.event_skill_id, 0);
    }
}
