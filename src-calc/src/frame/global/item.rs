use crate::frame::r#enum::tostr::ItemTrinket as TrinketField;
use pak::{tab_get, tab_init};

use log::error;
use once_cell::sync::Lazy;

// TODO: 目前仅完成了特效腰坠

/* static manager variable */
static TRINKET: Lazy<super::Manager<i32, Item>> = Lazy::new(super::Manager::new);

/* struct */

/// Item
struct Item {
    id: i32,
    skill_id: i32,
    skill_level: i32,
    cooldown_id: i32,
}

/* impls */

impl Item {
    pub fn get(id: i32) -> Option<&'static Item> {
        TRINKET.get(&id)
    }
}

impl super::SubTrait<i32> for Item {
    fn struct_name() -> &'static str {
        "Item"
    }
    fn tab_init() {
        let fields = TrinketField::to_fields();
        let fields: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
        if !tab_init("settings/item/custom_trinket.tab", &["ID"], &fields) {
            error!("[global::item] Tab init failed");
        }
    }
    fn construct_from_tab(key: &i32) -> Option<Self> {
        let res = match tab_get("custom_trinket.tab", &[&key.to_string()]) {
            Ok(res) => res,
            Err(e) => {
                error!("[global::item] {:?} not found:\n{}", key, e);
                return None;
            }
        };
        parse_res(&res)
    }
}

fn parse_res(res: &[String]) -> Option<Item> {
    Some(Item {
        // `.ok()` should be used when the field is never an empty string.
        // `.unwrap_or()` should be used if compatibility with empty strings is required.
        id: res[TrinketField::ID as usize].parse().ok()?,
        skill_id: res[TrinketField::SkillID as usize].parse().unwrap_or(0), // Maybe empty string
        skill_level: res[TrinketField::SkillLevel as usize].parse().unwrap_or(0), // Maybe empty string
        cooldown_id: res[TrinketField::CoolDownID as usize].parse().unwrap_or(0), // Maybe empty string
    })
}

/* tests */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_pak() {
        let value = Item::get(27083).unwrap();
        assert_eq!(value.id, 27083);
        assert_eq!(value.skill_id, 6800);
        assert_eq!(value.skill_level, 47);
        assert_eq!(value.cooldown_id, 329);
    }
}
