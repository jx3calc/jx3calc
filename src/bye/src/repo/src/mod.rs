mod bye;

#[no_mangle]
pub extern "C" fn say_goodbye(name: *const std::os::raw::c_char) -> *mut std::os::raw::c_char {
    use std::ffi::{CStr, CString};
    let c_str = unsafe { CStr::from_ptr(name) };
    let r_str = c_str.to_str().unwrap();
    let response = bye::say_goodbye(r_str);
    CString::new(response).unwrap().into_raw()
}
