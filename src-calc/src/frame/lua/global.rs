use super::_mod;

#[allow(non_snake_case)]
pub(super) fn GetEditorString(_: &mlua::Lua, (a, b): (i32, i32)) -> mlua::Result<String> {
    Ok(format!("{}_{}", a, b))
}

#[allow(non_snake_case)]
pub(super) fn Include(_: &mlua::Lua, filename: String) -> mlua::Result<()> {
    _mod::load(&filename)
}

#[allow(non_snake_case)]
pub(super) fn IsClient(_: &mlua::Lua, _: ()) -> mlua::Result<bool> {
    Ok(false)
}
