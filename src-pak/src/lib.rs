mod jx3pak;

use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

/// ##### About legality of borrowing a temporary value:
/// See: https://stackoverflow.com/questions/47662253/why-is-it-legal-to-borrow-a-temporary
/// In short, the following Rust code is legal:
/// ```
/// let x = &String::new();
/// ```
/// It has the same effect as the following Rust code:
/// ```
/// let a_variable_you_cant_see = String::new();
/// let x = &a_variable_you_cant_see;
/// ```
/// It means that the temporary value will be dropped after the reference is dropped automatically.
///
/// ##### About large vector:
/// use `vec![V; SIZE].into_boxed_slice()` to avoid stack overflow.

macro_rules! ensure_0 {
    ($s:expr) => {
        match $s.ends_with('\0') {
            true => $s,
            false => &($s.to_owned() + "\0"),
        }
    };
}

pub fn init(path: &str) -> bool {
    let path = ensure_0!(path);
    jx3pak::init(path.as_ptr() as *const c_char) == (0 as c_int)
}

pub fn tab_init(path: &str, indexs: &[&str], fields: &[&str]) -> bool {
    let path = ensure_0!(path);
    let indexs = indexs.join("\t") + "\t\0";
    let fields = fields.join("\t") + "\t\0";
    let ptr_path = path.as_ptr() as *const c_char;
    let ptr_indexs = indexs.as_ptr() as *const c_char;
    let ptr_fields = fields.as_ptr() as *const c_char;
    jx3pak::tab_init(ptr_path, ptr_indexs, ptr_fields) == (0 as c_int)
}

static RESULT_MAXLEN: usize = 1024 * 1024; // 1MB

pub fn tab_get(tabname: &str, key: &[&str]) -> std::io::Result<Vec<String>> {
    let tabname = ensure_0!(tabname);
    let key = key.join("\t") + "\t\0";
    let mut result = vec![0 as c_char; RESULT_MAXLEN].into_boxed_slice();
    let ptr_tabname = tabname.as_ptr() as *const c_char;
    let ptr_key = key.as_ptr() as *const c_char;
    let ptr_result = result.as_mut_ptr() as *mut c_char;
    let result_maxlen = RESULT_MAXLEN as c_int;
    let res_len = jx3pak::tab_get(ptr_tabname, ptr_key, ptr_result, result_maxlen);
    if res_len <= 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "[pak] Tab get failed internally",
        ));
    }
    let res = unsafe { CStr::from_ptr(ptr_result).to_str() };
    if res.is_err() || res.unwrap().len() != (res_len - 1) as usize {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "[pak] Tab get failed to convert result to string",
        ));
    }
    let mut res = res.unwrap();
    if res.ends_with("\t") {
        res = &res[..res.len() - 1];
    }
    Ok(res.split('\t').map(|s| s.to_string()).collect())
}

pub fn lua_get(path: &str) -> std::io::Result<Vec<u8>> {
    let path = ensure_0!(path);
    let mut result = vec![0 as c_char; RESULT_MAXLEN].into_boxed_slice();
    let ptr_path = path.as_ptr() as *const c_char;
    let ptr_result = result.as_mut_ptr() as *mut c_char;
    let result_maxlen = RESULT_MAXLEN as c_int;
    let res_len = jx3pak::lua_get(ptr_path, ptr_result, result_maxlen);
    if res_len <= 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "[pak] Lua get failed internally",
        ));
    }
    let res_slice =
        unsafe { std::slice::from_raw_parts(ptr_result as *const u8, res_len as usize) };
    Ok(res_slice.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua;

    #[test]
    fn test() {
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
}
