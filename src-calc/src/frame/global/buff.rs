use crate::frame::r#enum::{
    fromstr::Attrib as RefAttrib,
    tostr::{Buff as Field, BuffUI as UIField},
};
use pak::{tab_get, tab_init};

use log::error;
use once_cell::sync::Lazy;
use strum::IntoEnumIterator;

/* static manager variable */
static BUFF: Lazy<super::Manager<BuffKey, Buff>> = Lazy::new(super::Manager::new);

/* structs */

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
    max_stacknum: i32,
    count: i32,
    interval: i32,
    hide: bool,
    exclude: bool,
    script_file: String,
    can_cancel: bool,
    min_interval: i32,
    max_interval: i32,
    begin_attrib: Vec<Attrib>,
    active_attrib: Vec<Attrib>,
    end_time_attrib: Vec<Attrib>,
    ui: Option<UI>,
}

/* sub structs */

struct UI {
    id: i32,
    level: i32,
    name: String,
}

#[derive(Debug, PartialEq, Eq)]
enum AttribValue {
    Int(i32),
    String(String),
}

#[derive(Debug, PartialEq, Eq)]
struct Attrib {
    r#type: RefAttrib,
    value_a: AttribValue,
    value_b: AttribValue,
}

/* impls */

impl Buff {
    pub fn get(id: i32, level: i32) -> Option<&'static Buff> {
        let key = BuffKey { id, level };
        BUFF.get(&key)
    }
}

impl super::SubTrait<BuffKey> for Buff {
    fn struct_name() -> &'static str {
        "Buff"
    }
    fn tab_init() {
        let fields = Field::to_fields();
        let fields: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
        if !tab_init("settings/skill/buff.tab", &["ID", "Level"], &fields) {
            error!("[global::buff] Tab init failed");
        }
        UI::tab_init();
    }
    fn construct_from_tab(key: &BuffKey) -> Option<Self> {
        let res = match tab_get("buff.tab", &[&key.id.to_string(), &key.level.to_string()]) {
            Ok(res) => res,
            Err(e) => {
                error!("[global::buff] {:?} not found:\n{}", key, e);
                return None;
            }
        };
        let mut buff = parse_res(&res)?;
        buff.ui = UI::construct_from_tab(key);
        Some(buff)
    }
}

fn parse_res(res: &[String]) -> Option<Buff> {
    let mut buff = Buff {
        // `.ok()` should be used when the field is never an empty string.
        // `.unwrap_or()` should be used if compatibility with empty strings is required.
        id: res[Field::ID as usize].parse().ok()?,
        level: res[Field::Level as usize].parse().ok()?,
        is_stackable: res[Field::IsStackable as usize] == "1",
        max_stacknum: res[Field::MaxStackNum as usize].parse().ok()?,
        count: res[Field::Count as usize].parse().ok()?,
        interval: res[Field::Interval as usize].parse().ok()?,
        hide: res[Field::Hide as usize] == "1",
        exclude: res[Field::Exclude as usize] == "1",
        script_file: res[Field::ScriptFile as usize].clone(),
        can_cancel: res[Field::CanCancel as usize] == "1",
        min_interval: res[Field::MinInterval as usize].parse().ok()?,
        max_interval: res[Field::MaxInterval as usize].parse().ok()?,
        begin_attrib: Vec::new(),
        active_attrib: Vec::new(),
        end_time_attrib: Vec::new(),
        ui: None,
    };
    fn add_attrib(v: &mut Vec<Attrib>, res: &[String], begin: usize, count: usize) {
        use std::str::FromStr; // Required for Attrib::from_str
        for i in 0..count {
            let attrib = &res[begin + i * 3];
            if attrib.is_empty() {
                continue;
            }
            let attrib = match RefAttrib::from_str(attrib) {
                Ok(v) => v,
                Err(_) => {
                    error!("[global::buff] Unregistered attrib: {}", attrib);
                    continue;
                }
            };
            let va = match res[begin + i * 3 + 1].parse::<i32>() {
                Ok(v) => AttribValue::Int(v),
                Err(_) => AttribValue::String(res[begin + i * 3 + 1].clone()),
            };
            let vb = match res[begin + i * 3 + 2].parse::<i32>() {
                Ok(v) => AttribValue::Int(v),
                Err(_) => AttribValue::String(res[begin + i * 3 + 2].clone()),
            };
            v.push(Attrib {
                r#type: attrib,
                value_a: va,
                value_b: vb,
            });
        }
    }
    let count = Field::iter().count();
    add_attrib(&mut buff.begin_attrib, &res, count, Field::BEGIN);
    let count = count + Field::BEGIN * 3;
    add_attrib(&mut buff.active_attrib, &res, count, Field::ACTIVE);
    let count = count + Field::ACTIVE * 3;
    add_attrib(&mut buff.end_time_attrib, &res, count, Field::END_TIME);

    Some(buff)
}

impl super::SubTrait<BuffKey> for UI {
    fn struct_name() -> &'static str {
        "BuffUI"
    }
    fn tab_init() {
        let fields: Vec<String> = UIField::to_fields();
        let fields: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
        if !tab_init("ui/scheme/case/buff.txt", &["BuffID", "Level"], &fields) {
            error!("[global::buffui] Tab init failed");
        }
    }
    fn construct_from_tab(key: &BuffKey) -> Option<Self> {
        let res = tab_get("buff.txt", &[&key.id.to_string(), &key.level.to_string()]).ok()?;
        Some(UI {
            id: res[UIField::BuffID as usize].parse().ok()?,
            level: res[UIField::Level as usize].parse().ok()?,
            name: res[UIField::Name as usize].clone(),
        })
    }
}

/* tests */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_pak() {
        let value = Buff::get(101, 1).unwrap();
        assert_eq!(value.id, 101);
        assert_eq!(value.level, 1);
        assert_eq!(
            value.ui.as_ref().unwrap().name,
            "Buff=1，Debuff=2，Dot=3，Hot=4"
        );
    }

    #[test]
    fn from_res() {
        let res= "4052\t1\t0\t1\t1\t160\t0\t1\t明教\\明教_暗尘弥散_非战斗非正常消失.lua\t1\t160\t160\tatStealth\t\t\tatMoveSpeedPercent\t0\t\tatSkillEventHandler\t696\t\tatSkillEventHandler\t697\t\tatExecuteScript\tskill/明教/驱散隐身待机动作.lua\t\tatExecuteScript\tskill/明教/明教_暗尘弥散_5秒惩罚CD.lua\t\tatAddTransparencyValue\t-70\t\tatKnockedDownRate\t-1024\t\tatExecuteScript\tskill/明教/新无间影狱加Buff.lua\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\tatBeImmunisedStealthEnable\t\t\t\t\t\t\t\t\t\t\t";
        let res = res
            .split('\t')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let res_len = Field::iter().count() + (Field::BEGIN + Field::ACTIVE + Field::END_TIME) * 3;
        assert_eq!(res.len(), res_len);
        let value = parse_res(&res).unwrap();
        assert_eq!(value.id, 4052);
        assert_eq!(value.level, 1);
        assert_eq!(value.is_stackable, false);
        assert_eq!(value.max_stacknum, 1);
        assert_eq!(value.count, 1);
        assert_eq!(value.interval, 160);
        assert_eq!(value.hide, false);
        assert_eq!(value.exclude, true);
        assert_eq!(
            value.script_file,
            "明教\\明教_暗尘弥散_非战斗非正常消失.lua"
        );
        assert_eq!(value.can_cancel, true);
        assert_eq!(value.min_interval, 160);
        assert_eq!(value.max_interval, 160);
        let empty_string = "".to_string();
        assert_eq!(
            value.begin_attrib[0],
            Attrib {
                r#type: RefAttrib::atStealth,
                value_a: AttribValue::String(empty_string.clone()),
                value_b: AttribValue::String(empty_string.clone()),
            }
        );
        assert_eq!(
            value.begin_attrib[1],
            Attrib {
                r#type: RefAttrib::atMoveSpeedPercent,
                value_a: AttribValue::Int(0),
                value_b: AttribValue::String(empty_string.clone()),
            }
        );
        assert_eq!(
            value.begin_attrib[2],
            Attrib {
                r#type: RefAttrib::atSkillEventHandler,
                value_a: AttribValue::Int(696),
                value_b: AttribValue::String(empty_string.clone()),
            }
        );
        assert_eq!(
            value.begin_attrib[3],
            Attrib {
                r#type: RefAttrib::atSkillEventHandler,
                value_a: AttribValue::Int(697),
                value_b: AttribValue::String(empty_string.clone()),
            }
        );
        assert_eq!(
            value.begin_attrib[4],
            Attrib {
                r#type: RefAttrib::atExecuteScript,
                value_a: AttribValue::String("skill/明教/驱散隐身待机动作.lua".to_string()),
                value_b: AttribValue::String(empty_string.clone()),
            }
        );
        assert_eq!(
            value.begin_attrib[5],
            Attrib {
                r#type: RefAttrib::atExecuteScript,
                value_a: AttribValue::String("skill/明教/明教_暗尘弥散_5秒惩罚CD.lua".to_string()),
                value_b: AttribValue::String(empty_string.clone()),
            }
        );
        assert_eq!(
            value.begin_attrib[6],
            Attrib {
                r#type: RefAttrib::atAddTransparencyValue,
                value_a: AttribValue::Int(-70),
                value_b: AttribValue::String(empty_string.clone()),
            }
        );
        assert_eq!(
            value.begin_attrib[7],
            Attrib {
                r#type: RefAttrib::atKnockedDownRate,
                value_a: AttribValue::Int(-1024),
                value_b: AttribValue::String(empty_string.clone()),
            }
        );
        assert_eq!(
            value.begin_attrib[8],
            Attrib {
                r#type: RefAttrib::atExecuteScript,
                value_a: AttribValue::String("skill/明教/新无间影狱加Buff.lua".to_string()),
                value_b: AttribValue::String(empty_string.clone()),
            }
        );
        assert_eq!(
            value.active_attrib[0],
            Attrib {
                r#type: RefAttrib::atBeImmunisedStealthEnable,
                value_a: AttribValue::String(empty_string.clone()),
                value_b: AttribValue::String(empty_string.clone()),
            }
        );
    }
}
