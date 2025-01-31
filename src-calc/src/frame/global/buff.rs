use crate::frame::r#enum::{fromstr::Attrib as RefAttrib, tostr::Buff as Field};
use pak::{tab_get, tab_init};

use log::error;
use once_cell::sync::Lazy;
use strum::IntoEnumIterator;

/* static manager variable */
static BUFF: Lazy<super::Manager<(i32, i32), Buff>> = Lazy::new(super::Manager::new);

/* struct */

/// Buff
pub(super) struct Buff {
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
    name: Option<String>,
}

/* sub structs */

struct UI(String);

#[derive(Debug, PartialEq, Eq)]
enum AttribValue {
    Int(i32),
    String(String),
}

#[derive(Debug, Eq, PartialEq)]
struct Attrib {
    r#type: RefAttrib,
    value_a: AttribValue,
    value_b: AttribValue,
}

/* impls */

pub(super) fn get(id: i32, level: i32) -> Option<&'static Buff> {
    BUFF.get(&(id, level))
}

impl super::SubTrait<(i32, i32)> for Buff {
    fn struct_name() -> &'static str {
        "Buff"
    }
    fn tab_init() {
        let fields = Field::to_fields();
        let fields: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
        if !tab_init("settings/skill/buff.tab", &["ID", "Level"], &fields) {
            error!("Tab init failed");
        }
        UI::tab_init();
    }
    fn construct_from_tab(key: &(i32, i32)) -> Option<Vec<String>> {
        match tab_get("buff.tab", &[&key.0.to_string(), &key.1.to_string()]) {
            Ok(mut res) => {
                // 处理 UI
                if let Some(mut ui) = UI::construct_from_tab(key) {
                    res.append(&mut ui);
                } else {
                    let key = (key.0, 0);
                    if let Some(mut ui) = UI::construct_from_tab(&key) {
                        res.append(&mut ui);
                    }
                }
                Some(res)
            }
            Err(e) => {
                error!("{:?} not found:\n{}", key, e);
                None
            }
        }
    }
    fn parse_from_data(data: &[String]) -> Option<Buff> {
        let mut buff = Buff {
            // `.ok()` should be used when the field is never an empty string.
            // `.unwrap_or()` should be used if compatibility with empty strings is required.
            id: data[Field::ID as usize].parse().ok()?,
            level: data[Field::Level as usize].parse().ok()?,
            is_stackable: data[Field::IsStackable as usize] == "1",
            max_stacknum: data[Field::MaxStackNum as usize].parse().ok()?,
            count: data[Field::Count as usize].parse().ok()?,
            interval: data[Field::Interval as usize].parse().ok()?,
            hide: data[Field::Hide as usize] == "1",
            exclude: data[Field::Exclude as usize] == "1",
            script_file: data[Field::ScriptFile as usize].clone(),
            can_cancel: data[Field::CanCancel as usize] == "1",
            min_interval: data[Field::MinInterval as usize].parse().ok()?,
            max_interval: data[Field::MaxInterval as usize].parse().ok()?,
            begin_attrib: Vec::new(),
            active_attrib: Vec::new(),
            end_time_attrib: Vec::new(),
            name: None,
        };
        // 处理 Attrib
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
                        error!("Unregistered attrib: {}", attrib);
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
        add_attrib(&mut buff.begin_attrib, &data, count, Field::BEGIN);
        let count = count + Field::BEGIN * 3;
        add_attrib(&mut buff.active_attrib, &data, count, Field::ACTIVE);
        let count = count + Field::ACTIVE * 3;
        add_attrib(&mut buff.end_time_attrib, &data, count, Field::END_TIME);
        let count = count + Field::END_TIME * 3;

        //  处理 UI
        if count < data.len() {
            buff.name = UI::parse_from_data(&data[count..]).map(|ui| ui.0);
        }

        Some(buff)
    }
}

impl super::SubTrait<(i32, i32)> for UI {
    fn struct_name() -> &'static str {
        "BuffUI"
    }
    fn tab_init() {
        let fields = vec!["Name"];
        if !tab_init("ui/scheme/case/buff.txt", &["BuffID", "Level"], &fields) {
            error!("Tab init failed");
        }
    }
    fn construct_from_tab(key: &(i32, i32)) -> Option<Vec<String>> {
        match tab_get("buff.txt", &[&key.0.to_string(), &key.1.to_string()]) {
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

/* tests */
#[cfg(test)]
mod tests {
    use super::super::SubTrait;
    use super::*;

    #[test]
    fn from_pak() {
        let value = get(101, 1).unwrap();
        assert_eq!(value.id, 101);
        assert_eq!(value.level, 1);
        assert_eq!(
            value.name.as_ref().unwrap(),
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
        let value = Buff::parse_from_data(&res).unwrap();
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
