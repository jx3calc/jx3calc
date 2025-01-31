use super::global;
use crate::frame::r#enum::xlua;
use mlua;
use pak::lua_get;
use strum::{EnumCount, IntoEnumIterator};

thread_local! {
     static INSTANCE: LuaManager = LuaManager::new();
}
static SIZE: usize = xlua::FuncName::COUNT;

struct LuaManager {
    lua: mlua::Lua,
    map: std::cell::RefCell<std::collections::HashMap<String, [Option<mlua::Function>; SIZE]>>,
}

pub(crate) fn get_func(
    scriptfile: &str,
    func: xlua::FuncName,
) -> mlua::Result<Option<mlua::Function>> {
    INSTANCE.with(|t| {
        let key = normalize_file_name(scriptfile);
        t.load(&key)?;
        Ok(t.map.borrow().get(&key).unwrap()[func as usize].clone()) // safe: key is loaded just now
    })
}

pub(super) fn load(scriptfile: &str) -> mlua::Result<()> {
    INSTANCE.with(|t| t.load(scriptfile))
}

pub(crate) fn scope<'env, R>(
    f: impl for<'scope> FnOnce(&'scope mut mlua::Scope<'scope, 'env>) -> mlua::Result<R>,
) -> mlua::Result<R> {
    INSTANCE.with(|t| t.lua.scope(f))
}

impl LuaManager {
    fn new() -> Self {
        let instance = Self {
            lua: mlua::Lua::new(),
            map: std::cell::RefCell::new(std::collections::HashMap::new()),
        };
        instance.init();
        instance
    }

    pub(super) fn load(&self, scriptfile: &str) -> mlua::Result<()> {
        // 检查是否已加载
        let key = normalize_file_name(scriptfile);
        if self.map.borrow().contains_key(&key) {
            return Ok(());
        }
        // 准备数据
        let data = lua_get(scriptfile)?;

        // let name = filename.split('/').last().unwrap();
        // let mut file = std::fs::File::create(name)?;
        // std::io::Write::write_all(&mut file, &data)?;

        // 初始化
        for it in xlua::FuncName::iter() {
            self.lua.globals().set(it.to_string(), mlua::Value::Nil)?;
        }
        // 执行
        let res = self.lua.load(&data).exec();
        res.map_err(|e| mlua::Error::runtime(format!("{} exec failed:\n{}", scriptfile, e)))?;
        // 获取
        let mut value = [const { None }; SIZE];
        for it in xlua::FuncName::iter() {
            let res: mlua::Value = self.lua.globals().get(it.to_string())?;
            value[it as usize] = res.as_function().cloned();
        }
        // 保存
        self.map.borrow_mut().insert(key, value);
        Ok(())
    }

    fn init(&self) {
        macro_rules! set_func {
            ($name:ident) => {
                let f = self.lua.create_function(global::$name).unwrap();
                self.lua.globals().set(stringify!($name), f).unwrap();
            };
        }
        macro_rules! set_table {
            ($name:ident) => {
                let t = self.lua.create_table().unwrap();
                for it in xlua::$name::iter() {
                    t.set(it.to_string(), it as u8).unwrap();
                }
                self.lua.globals().set(stringify!($name), t).unwrap();
            };
        }
        set_func!(GetEditorString);
        set_func!(Include);
        set_func!(IsClient);
        set_table!(ABSORB_ATTRIBUTE_SHIELD_TYPE);
        set_table!(ATTRIBUTE_EFFECT_MODE);
        set_table!(BUFF_COMPARE_FLAG);
        set_table!(SKILL_COMPARE_FLAG);
        set_table!(ATTRIBUTE_TYPE);
        set_table!(GLOBAL);
    }
}

fn normalize_file_name(source: &str) -> String {
    source
        .trim()
        .replace('\\', "/")
        .replace("//", "/")
        .trim_matches('/')
        .to_lowercase()
}

// #[test]
// fn debug() {
//     let filename = "./test.lua";
//     let data = std::fs::read(filename).unwrap();
//     INSTANCE.with(|t| {
//         t.lua.load(&data).exec().unwrap();
//     });
// }

#[cfg(test)]
mod mlua_tests {
    use super::*;

    #[test]
    fn test_nil() {
        let lua = mlua::Lua::new();
        let lua_func: Result<mlua::Function, mlua::Error> = lua.globals().get("rust_func");
        assert!(lua_func.is_err());
        let lua_func: mlua::Value = lua.globals().get("rust_func").unwrap();
        assert!(lua_func.is_nil());
        let rust_func = lua
            .create_function(|_, (a, b): (i32, i32)| Ok(a + b))
            .unwrap();
        lua.globals().set("rust_func", rust_func).unwrap();
        let lua_func: mlua::Value = lua.globals().get("rust_func").unwrap();
        assert!(lua_func.is_function());
    }

    #[test]
    fn test_create() {
        fn func(_: &mlua::Lua, (a, b): (i32, i32)) -> mlua::Result<i32> {
            Ok(a + b)
        }
        let lua = mlua::Lua::new();
        let rust_func = lua.create_function(func).unwrap();
        lua.globals().set("rust_func", rust_func).unwrap();
        let lua_func: mlua::Function = lua.globals().get("rust_func").unwrap();
        let res = lua_func.call::<i32>((114, 514)).unwrap();
        assert_eq!(res, 114 + 514);
    }

    #[test]
    fn create_closure() {
        let func = |_: &mlua::Lua, value: i32| -> mlua::Result<i32> { Ok(value * 2) };
        let lua = mlua::Lua::new();
        let rust_func = lua.create_function(func).unwrap();
        lua.globals().set("rust_func", rust_func).unwrap();
        let lua_func: mlua::Function = lua.globals().get("rust_func").unwrap();
        let res = lua_func.call::<i32>(114514).unwrap();
        assert_eq!(res, 114514 * 2);
    }

    static CLOSURE: fn(&mlua::Lua, i32) -> mlua::Result<i32> =
        |_: &mlua::Lua, value: i32| -> mlua::Result<i32> { Ok(value * 2) };
    #[test]
    fn create_global_closure() {
        let lua = mlua::Lua::new();
        let rust_func = lua.create_function(CLOSURE).unwrap();
        lua.globals().set("rust_func", rust_func).unwrap();
        let lua_func: mlua::Function = lua.globals().get("rust_func").unwrap();
        let res = lua_func.call::<i32>(114514).unwrap();
        assert_eq!(res, 114514 * 2);
    }

    fn func1(_: &mlua::Lua, value: i32) -> mlua::Result<i32> {
        Ok(-value)
    }
    fn func2(_: &mlua::Lua, value: i32) -> mlua::Result<i32> {
        Ok(value * 2)
    }

    #[test]
    fn test_override() {
        let lua = mlua::Lua::new();
        let rust_func1 = lua.create_function(func1).unwrap();
        let rust_func2 = lua.create_function(func2).unwrap();
        lua.globals().set("rust_func", rust_func1).unwrap();
        lua.globals().set("rust_func", rust_func2).unwrap();
        let lua_func: mlua::Function = lua.globals().get("rust_func").unwrap();
        let x = lua_func.call::<i32>(114514).unwrap();
        assert_eq!(x, 114514 * 2);
    }

    #[test]
    fn get_before_override() {
        let lua = mlua::Lua::new();
        let rust_func1 = lua.create_function(func1).unwrap();
        let rust_func2 = lua.create_function(func2).unwrap();
        lua.globals().set("rust_func", rust_func1).unwrap();
        let lua_func: mlua::Function = lua.globals().get("rust_func").unwrap();
        lua.globals().set("rust_func", rust_func2).unwrap();
        let x = lua_func.call::<i32>(114514).unwrap();
        assert_eq!(x, -114514);
    }

    #[test]
    fn test_clone() {
        let lua = mlua::Lua::new();
        let rust_func = lua.create_function(func1).unwrap();
        lua.globals().set("rust_func", rust_func).unwrap();
        let lua_func: mlua::Function = lua.globals().get("rust_func").unwrap();
        let func_clone = lua_func.clone();
        let rust_func = lua.create_function(func2).unwrap();
        lua.globals().set("rust_func", rust_func).unwrap();
        let lua_func: mlua::Function = lua.globals().get("rust_func").unwrap();
        assert_eq!(func_clone.call::<i32>(114514).unwrap(), -114514);
        assert_eq!(lua_func.call::<i32>(114514).unwrap(), 114514 * 2);
    }
}
