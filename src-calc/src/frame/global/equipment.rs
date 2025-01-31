use crate::frame::r#enum::tostr::CustomTrinket as TrinketField;
use pak::{tab_get, tab_init};

use log::error;
use once_cell::sync::Lazy;

// TODO: 目前仅完成了特效腰坠

/* static manager variable */
static TRINKET: Lazy<super::Manager<i32, Equipment>> = Lazy::new(super::Manager::new);

/* struct */

/// Equipment
pub(super) struct Equipment {
    id: i32,
    skill_id: i32,
    skill_level: i32,
    cooldown_id: i32,
}

/* impls */

pub(super) fn get(id: i32) -> Option<&'static Equipment> {
    TRINKET.get(&id)
}

impl super::SubTrait<i32> for Equipment {
    fn struct_name() -> &'static str {
        "Equipment"
    }
    fn tab_init() {
        let fields = TrinketField::to_fields();
        let fields: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
        if !tab_init("settings/item/custom_trinket.tab", &["ID"], &fields) {
            error!("Tab init failed: Trinket");
        }
    }
    fn construct_from_tab(key: &i32) -> Option<Vec<String>> {
        match tab_get("custom_trinket.tab", &[&key.to_string()]) {
            Ok(res) => Some(res),
            Err(e) => {
                error!("{:?} not found:\n{}", key, e);
                None
            }
        }
    }
    fn parse_from_data(data: &[String]) -> Option<Equipment> {
        Some(Equipment {
            // `.ok()` should be used when the field is never an empty string.
            // `.unwrap_or()` should be used if compatibility with empty strings is required.
            id: data[TrinketField::ID as usize].parse().ok()?,
            skill_id: data[TrinketField::SkillID as usize].parse().unwrap_or(0), // Maybe empty string
            skill_level: data[TrinketField::SkillLevel as usize].parse().unwrap_or(0), // Maybe empty string
            cooldown_id: data[TrinketField::CoolDownID as usize].parse().unwrap_or(0), // Maybe empty string
        })
    }
}

/* tests */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_pak() {
        let value = get(27083).unwrap();
        assert_eq!(value.id, 27083);
        assert_eq!(value.skill_id, 6800);
        assert_eq!(value.skill_level, 47);
        assert_eq!(value.cooldown_id, 329);
    }
}
