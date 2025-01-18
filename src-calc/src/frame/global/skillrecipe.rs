use crate::frame::r#enum::tostr::SkillRecipe as Field;
use pak::{tab_get, tab_init};

use log::error;
use once_cell::sync::Lazy;

// TODO: 目前没有做技能处理

/* static manager variable */
static SKILL_RECIPE: Lazy<super::Manager<SkillRecipeKey, SkillRecipe>> =
    Lazy::new(super::Manager::new);

/* structs */

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SkillRecipeKey {
    id: i32,
    level: i32,
}

/// SkillRecipe
struct SkillRecipe {
    recipe_id: i32,
    recipe_level: i32,
    skill_recipe_type: i32,
    skill_id: i32,
    cool_down_add1: i32,
    cool_down_add2: i32,
    cool_down_add3: i32,
    damage_add_percent: i32,
    has_script_file: bool,
}

/* impls */

impl SkillRecipe {
    pub fn get(id: i32, level: i32) -> Option<&'static SkillRecipe> {
        let key = SkillRecipeKey { id, level };
        SKILL_RECIPE.get(&key)
    }
}

impl super::SubTrait<SkillRecipeKey> for SkillRecipe {
    fn struct_name() -> &'static str {
        "SkillRecipe"
    }
    fn tab_init() {
        let fields = Field::to_fields();
        let fields: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
        if !tab_init(
            "settings/skill/recipeskill.tab",
            &["RecipeID", "RecipeLevel"],
            &fields,
        ) {
            error!("[global::skillrecipe] Tab init failed");
        }
    }
    fn construct_from_tab(key: &SkillRecipeKey) -> Option<Self> {
        let res = match tab_get(
            "recipeskill.tab",
            &[&key.id.to_string(), &key.level.to_string()],
        ) {
            Ok(res) => res,
            Err(e) => {
                error!("[global::skillrecipe] {:?} not found:\n{}", key, e);
                return None;
            }
        };
        parse_res(&res)
    }
}

fn parse_res(res: &[String]) -> Option<SkillRecipe> {
    Some(SkillRecipe {
        // `.ok()` should be used when the field is never an empty string.
        // `.unwrap_or()` should be used if compatibility with empty strings is required.
        recipe_id: res[Field::RecipeID as usize].parse().ok()?,
        recipe_level: res[Field::RecipeLevel as usize].parse().ok()?,
        skill_recipe_type: res[Field::SkillRecipeType as usize].parse().unwrap_or(0),
        skill_id: res[Field::SkillID as usize].parse().unwrap_or(0),
        cool_down_add1: res[Field::CoolDownAdd1 as usize].parse().unwrap_or(0),
        cool_down_add2: res[Field::CoolDownAdd2 as usize].parse().unwrap_or(0),
        cool_down_add3: res[Field::CoolDownAdd3 as usize].parse().unwrap_or(0),
        damage_add_percent: res[Field::DamageAddPercent as usize].parse().unwrap_or(0),
        has_script_file: !res[Field::ScriptFile as usize].is_empty(),
    })
}

/* tests */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_pak() {
        let value = SkillRecipe::get(1, 1).unwrap();
        assert_eq!(value.recipe_id, 1);
        assert_eq!(value.recipe_level, 1);
        assert_eq!(value.skill_recipe_type, 0);
        assert_eq!(value.skill_id, 55);
        assert_eq!(value.cool_down_add1, -240);
        assert_eq!(value.cool_down_add2, -16);
        assert_eq!(value.cool_down_add3, 0);
        assert_eq!(value.damage_add_percent, 0);
        assert_eq!(value.has_script_file, false);
    }
}
