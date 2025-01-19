use crate::frame::r#enum::tostr::SkillRecipe as Field;
use pak::{tab_get, tab_init};

use log::error;
use once_cell::sync::Lazy;

// TODO: 目前没有做技能处理

/* static manager variable */
static SKILL_RECIPE: Lazy<super::Manager<(i32, i32), SkillRecipe>> = Lazy::new(super::Manager::new);

/* struct */

/// SkillRecipe
pub struct SkillRecipe {
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

pub fn get(id: i32, level: i32) -> Option<&'static SkillRecipe> {
    SKILL_RECIPE.get(&(id, level))
}

impl super::SubTrait<(i32, i32)> for SkillRecipe {
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
    fn construct_from_tab(key: &(i32, i32)) -> Option<Self> {
        let res = match tab_get("recipeskill.tab", &[&key.0.to_string(), &key.1.to_string()]) {
            Ok(res) => res,
            Err(e) => {
                error!("[global::skillrecipe] {:?} not found:\n{}", key, e);
                return None;
            }
        };
        Self::parse_from_data(&res)
    }
    fn parse_from_data(data: &[String]) -> Option<SkillRecipe> {
        Some(SkillRecipe {
            // `.ok()` should be used when the field is never an empty string.
            // `.unwrap_or()` should be used if compatibility with empty strings is required.
            recipe_id: data[Field::RecipeID as usize].parse().ok()?,
            recipe_level: data[Field::RecipeLevel as usize].parse().ok()?,
            skill_recipe_type: data[Field::SkillRecipeType as usize].parse().unwrap_or(0),
            skill_id: data[Field::SkillID as usize].parse().unwrap_or(0),
            cool_down_add1: data[Field::CoolDownAdd1 as usize].parse().unwrap_or(0),
            cool_down_add2: data[Field::CoolDownAdd2 as usize].parse().unwrap_or(0),
            cool_down_add3: data[Field::CoolDownAdd3 as usize].parse().unwrap_or(0),
            damage_add_percent: data[Field::DamageAddPercent as usize].parse().unwrap_or(0),
            has_script_file: !data[Field::ScriptFile as usize].is_empty(),
        })
    }
}

/* tests */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_pak() {
        let value = get(1, 1).unwrap();
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
