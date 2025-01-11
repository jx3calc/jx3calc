use pak::*;

fn main() {
    match std::env::var("v4_path") {
        Ok(v4_path) => assert!(init(&v4_path)), // v4
        Err(_) => assert!(init("./cache")),     // v5
    };

    assert!(tab_init(
        "settings/skill/buff.tab",
        &["ID", "Level"],
        &["ID", "Level", "Name", "Interval"]
    ));
    let tab_res = tab_get("buff.tab", &["101", "1"]).unwrap();
    assert_eq!(tab_res, vec!["101", "1", "策划默认项(非程序默认行)", "0"]);

    let lua_res = lua_get("scripts\\skill\\Default.lua").unwrap();
    let lua51_header = [0x1B, 0x4C, 0x75, 0x61];
    assert_eq!(lua_res[0..4], lua51_header);

    let lua = mlua::Lua::new();
    let mut included: String = String::new();
    let _ = lua.scope(|scope| {
        let func_include = scope
            .create_function_mut(|_, path: String| {
                included = path;
                Ok(())
            })
            .unwrap();
        lua.globals().set("Include", func_include).unwrap();
        lua.load(&lua_res).exec().unwrap();
        Ok(())
    });
    assert_eq!(included, "scripts/Include/Skill.lh");
}
