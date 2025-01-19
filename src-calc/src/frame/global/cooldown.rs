use crate::frame::r#enum::tostr::Cooldown as Field;
use pak::{tab_get, tab_init};

use log::error;
use once_cell::sync::Lazy;

/* static manager variable */
static COOLDOWN: Lazy<super::Manager<i32, Cooldown>> = Lazy::new(super::Manager::new);

/* struct */

/// Cooldown
pub struct Cooldown {
    id: i32,
    duration_frame: i32,
    min_duration_frame: i32,
    max_duration_frame: i32,
    max_count: i32,
}

/* impls */

pub fn get(id: i32) -> Option<&'static Cooldown> {
    COOLDOWN.get(&id)
}

impl super::SubTrait<i32> for Cooldown {
    fn struct_name() -> &'static str {
        "Cooldown"
    }
    fn tab_init() {
        let fields = Field::to_fields();
        let fields: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
        if !tab_init("settings/cooldownlist.tab", &["ID"], &fields) {
            error!("[global::cooldown] Tab init failed");
        }
    }
    fn construct_from_tab(key: &i32) -> Option<Self> {
        let res = match tab_get("cooldownlist.tab", &[&key.to_string()]) {
            Ok(res) => res,
            Err(e) => {
                error!("[global::cooldown] {:?} not found:\n{}", key, e);
                return None;
            }
        };
        Self::parse_from_data(&res)
    }
    fn parse_from_data(data: &[String]) -> Option<Cooldown> {
        // `.ok()` should be used when the field is never an empty string.
        // `.unwrap_or()` should be used if compatibility with empty strings is required.
        let duration: f64 = data[Field::Duration as usize].parse().ok()?;
        let min_duration: f64 = data[Field::MinDuration as usize].parse().ok()?;
        let max_duration: f64 = data[Field::MaxDuration as usize].parse().ok()?;
        Some(Cooldown {
            id: data[Field::ID as usize].parse().ok()?,
            duration_frame: (duration * 16.0).round() as i32,
            min_duration_frame: (min_duration * 16.0).round() as i32,
            max_duration_frame: (max_duration * 16.0).round() as i32,
            max_count: data[Field::MaxCount as usize].parse().ok()?,
        })
    }
}

/* tests */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_pak() {
        let value = get(8).unwrap();
        assert_eq!(value.id, 8);
        assert_eq!(value.duration_frame, 8);
        assert_eq!(value.min_duration_frame, 8);
        assert_eq!(value.max_duration_frame, 8);
        assert_eq!(value.max_count, 1);
    }
}
