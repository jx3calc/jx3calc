use super::INSTANCE;

#[allow(non_snake_case)]
pub fn GetEditorString(_: &mlua::Lua, (a, b): (i32, i32)) -> mlua::Result<String> {
    Ok(format!("{}_{}", a, b))
}

#[allow(non_snake_case)]
pub fn Include(_: &mlua::Lua, filename: String) -> mlua::Result<()> {
    INSTANCE.with(|t| t.load(&filename))
}

#[allow(non_snake_case)]
pub fn IsClient(_: &mlua::Lua, _: ()) -> mlua::Result<bool> {
    Ok(false)
}
