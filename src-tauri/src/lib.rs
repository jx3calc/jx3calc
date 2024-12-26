#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("The answer is {}.", add::add(40, 2));
    let name = std::ffi::CString::new("jx3calc tauri").unwrap();
    let message_ptr = pak::say_hello(name.as_ptr());
    let message = unsafe { std::ffi::CStr::from_ptr(message_ptr).to_str().unwrap() };
    println!("{}", message);
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
