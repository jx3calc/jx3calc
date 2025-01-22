use super::{create_table, get_func, scope};
use crate::frame::{global::skill::Skill, r#enum::xlua::FuncName};

use mlua;

impl Skill {
    pub(crate) fn get_skill_level_data(&mut self, scriptfile: &str) -> mlua::Result<()> {
        let lua_func = get_func(scriptfile, FuncName::GetSkillLevelData)?.ok_or(
            mlua::Error::external(format!(
                "[global::skill] No GetSkillLevelData function in {}",
                scriptfile
            )),
        )?;

        let skill = create_table()?;
        let meta = create_table()?;
        type Param = (mlua::Value, mlua::Value, mlua::Value, mlua::Value);
        scope(|scope| {
            let add_attribue =
                scope.create_function(|_, (a, b, c, d): Param| -> mlua::Result<()> {
                    println!("AddAttribute: {:?} {:?} {:?} {:?}", a, b, c, d);
                    Ok(())
                })?;
            skill.set("AddAttribute", add_attribue)?;

            let getter = scope.create_function(|_, (_, key): (mlua::Table, String)| {
                println!("get: {:?}", key);
                match key.as_str() {
                    "dwLevel" => Ok(mlua::Value::Integer(1024)),
                    _ => Ok(mlua::Value::Nil),
                }
            })?;
            let setter = scope.create_function(
                |_, (_, key, value): (mlua::Table, String, mlua::Value)| {
                    println!("set: {:?} {:?}", key, value);
                    Ok(())
                },
            )?;
            meta.set("__index", getter).unwrap();
            meta.set("__newindex", setter).unwrap();
            skill.set_metatable(Some(meta));

            lua_func.call::<mlua::Value>(skill)?;
            Ok(())
        })?;
        Ok(())
    }
}
