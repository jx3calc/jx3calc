fn main() {
    let name = std::ffi::CString::new("jx3calc").unwrap();
    let message_ptr = bye::say_goodbye(name.as_ptr());
    let message = unsafe { std::ffi::CStr::from_ptr(message_ptr).to_str().unwrap() };
    println!("{}", message);
}
