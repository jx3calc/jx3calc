#[cfg(feature = "lib_local")]
mod src;
#[cfg(feature = "lib_local")]
pub use src::*;

#[cfg(not(feature = "lib_local"))]
use std::os::raw::{c_char, c_int};

macro_rules! external_unsafe {
    ($name:ident, $ret:ty, $( $arg:ident : $arg_ty:ty ),*) => {
        #[cfg(not(feature = "lib_local"))]
        pub extern "C" fn $name($( $arg : $arg_ty ),*) -> $ret {
            extern "C" {
                fn $name($( $arg : $arg_ty ),*) -> $ret;
            }
            unsafe { $name($( $arg ),*) }
        }
    };
}

external_unsafe!(init, c_int, path: *const c_char);
external_unsafe!(tab_init, c_int, path: *const c_char, indexs: *const c_char, fields: *const c_char);
external_unsafe!(tab_get, c_int, tabname: *const c_char, key: *const c_char, result: *mut c_char, result_maxlen: c_int);
external_unsafe!(lua_get, c_int, path: *const c_char, result: *mut c_char, result_maxlen: c_int);
