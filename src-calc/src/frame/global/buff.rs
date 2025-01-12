use super::super::enumeration;
use once_cell::sync::Lazy;
use pak;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

static BUFF: Lazy<super::Manager<BuffKey, Buff>> = Lazy::new(super::Manager::new);

/// Buff
struct Buff {
    id: i32,
    level: i32,
    name: String,
    interval: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BuffKey {
    id: i32,
    level: i32,
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
    fn tab_get(key: &BuffKey) -> Self {
        let res = pak::tab_get("buff.tab", &[&key.id.to_string(), &key.level.to_string()])
            .expect(&format!("[global::buff] Tab get failed for key {:?}", key));
        Buff {
            id: res[Field::ID as usize].parse().unwrap(),
            level: res[Field::Level as usize].parse().unwrap(),
            name: res[Field::Name as usize].clone(),
            interval: res[Field::Interval as usize].parse().unwrap(),
        }
    }
}

impl Buff {
    fn get(id: i32, level: i32) -> &'static Buff {
        let key = BuffKey { id, level };
        BUFF.get(&key)
    }
}

#[allow(non_camel_case_types)]
#[derive(EnumIter, Display)]
enum Field {
    ID,
    Level,
    Name,
    Interval,
}

impl Field {
    fn to_fields() -> Vec<String> {
        Field::iter().map(|x| x.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buff() {
        let value = Buff::get(101, 1);
        assert_eq!(value.id, 101);
        assert_eq!(value.level, 1);
        assert_eq!(value.name, "策划默认项(非程序默认行)");
        assert_eq!(value.interval, 0);
    }
}
