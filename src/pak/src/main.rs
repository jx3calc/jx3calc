use pak;

use std::os::raw::c_char;

fn rust2c_str(s: &str) -> *const c_char {
    std::ffi::CString::new(s).unwrap().into_raw()
}

fn rust2c_strs(vec: &Vec<&str>) -> Vec<*const c_char> {
    vec.iter().map(|s| rust2c_str(s)).collect()
}

fn c_str_to_vec(s: &[c_char], len: usize) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;
    for i in 0..len {
        if s[i] == 0 {
            if start == i {
                result.push("");
            } else if start < i {
                let slice = unsafe {
                    std::ffi::CStr::from_ptr(&s[start] as *const c_char)
                        .to_str()
                        .unwrap()
                };
                result.push(slice);
            }
            start = i + 1;
        }
    }
    result
}

fn main() {
    let path = "settings/skill/buff.tab";
    let index = vec!["buffID", "buffLevel"];
    let cols = vec!["buffID", "buffLevel", "buffName", "buffDesc"];
    let path = rust2c_str(path);
    let index = rust2c_strs(&index);
    let cols = rust2c_strs(&cols);
    let result = pak::tab_init(path, index.as_ptr(), index.len(), cols.as_ptr(), cols.len());
    println!("Result: {}\n", result);

    let i = 1;
    let index = vec!["1001", "1"];
    let index = rust2c_strs(&index);
    let mut res = [0 as c_char; 1024];
    let result = pak::tab_get(i, index.as_ptr(), index.len(), res.as_mut_ptr(), res.len());
    let res_vec = c_str_to_vec(&res, result as usize);
    println!("Result: {}", result);
    for s in res_vec {
        println!("{}#", s);
    }
    println!();

    let path = "settings/skill/buff.tab";
    let path = rust2c_str(path);
    let mut res = [0 as c_char; 1024];
    let result = pak::lua_get(path, res.as_mut_ptr(), res.len());
    let s = unsafe { std::ffi::CStr::from_ptr(res.as_ptr()).to_str().unwrap() };
    println!("Result: {}#{}#", result, s);
}
