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

external_unsafe!(say_hello, *mut std::os::raw::c_char, name: *const std::os::raw::c_char);
