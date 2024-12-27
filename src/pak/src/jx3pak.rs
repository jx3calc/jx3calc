#[cfg(feature = "lib_local")]
pub mod src;
#[cfg(feature = "lib_local")]
pub use src::*;

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

external_unsafe!(tab_init, c_int, path: *const c_char, index: *const *const c_char, index_len: usize, cols: *const *const c_char, cols_len: usize);
external_unsafe!(tab_get, c_int, i: c_int, index: *const *const c_char, index_len: usize, result: *mut c_char, result_maxlen: usize);
external_unsafe!(lua_get, c_int, path: *const c_char, result: *mut c_char, result_maxlen: usize);
