#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("The answer is {}.", add::add(40, 2));
    println!("{}", hello::say_hello("jx3calc tauri".to_string()));
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
