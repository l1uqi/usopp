// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod init;
mod tray;
mod event;

fn main() {
    init::init_app();
    tauri::Builder::default()
        .setup(|app| {
            event::resiger_event(&app);
            Ok(())
        })
        .system_tray(tauri::SystemTray::new().with_menu(tray::tray_menu()))
        .on_system_tray_event(tray::tray_event)
        .invoke_handler(tauri::generate_handler![
            command::search,
            command::open,
            command::window_change
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
