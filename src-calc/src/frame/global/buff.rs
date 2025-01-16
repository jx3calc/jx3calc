use super::super::enumeration;
use log::error;
use once_cell::sync::Lazy;
use pak;

static BUFF: Lazy<super::Manager<BuffKey, Buff>> = Lazy::new(super::Manager::new);

enum_field!(
    ID,
    Level,
    IsStackable,
    MaxStackNum,
    Count,
    Interval,
    Hide,
    Exclude,
    ScriptFile,
    CanCancel,
    MinInterval,
    MaxInterval
);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BuffKey {
    id: i32,
    level: i32,
}

/// Buff
struct Buff {
    id: i32,
    level: i32,
    is_stackable: bool,
    max_stack_num: i32,
    count: i32,
    interval: i32,
    hide: bool,
    exclude: bool,
    script_file: String,
    can_cancel: bool,
    min_interval: i32,
    max_interval: i32,
}

impl super::SubTrait<BuffKey> for Buff {
    fn struct_name() -> &'static str {
        "Buff"
    }
    fn tab_init() {
        let fields = Field::to_fields();
        let fields: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
        if !pak::tab_init("settings/skill/buff.tab", &["ID", "Level"], &fields) {
            panic!("[global::buff] Tab init failed");
        }
    }
    fn tab_get(key: &BuffKey) -> Option<Self> {
        let res = match pak::tab_get("buff.tab", &[&key.id.to_string(), &key.level.to_string()]) {
            Ok(res) => res,
            Err(e) => {
                error!("[global::buff] {:?} not found:\n{}", key, e);
                return None;
            }
        };
        Some(Buff {
            id: res[Field::ID as usize].parse().ok()?,
            level: res[Field::Level as usize].parse().ok()?,
            is_stackable: res[Field::IsStackable as usize] == "1",
            max_stack_num: res[Field::MaxStackNum as usize].parse().ok()?,
            count: res[Field::Count as usize].parse().ok()?,
            interval: res[Field::Interval as usize].parse().ok()?,
            hide: res[Field::Hide as usize] == "1",
            exclude: res[Field::Exclude as usize] == "1",
            script_file: res[Field::ScriptFile as usize].clone(),
            can_cancel: res[Field::CanCancel as usize] == "1",
            min_interval: res[Field::MinInterval as usize].parse().ok()?,
            max_interval: res[Field::MaxInterval as usize].parse().ok()?,
        })
    }
}

impl Buff {
    fn get(id: i32, level: i32) -> Option<&'static Buff> {
        let key = BuffKey { id, level };
        BUFF.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buff() {
        let value = Buff::get(101, 1).unwrap();
        assert_eq!(value.id, 101);
        assert_eq!(value.level, 1);
        assert_eq!(value.interval, 0);
    }
}
