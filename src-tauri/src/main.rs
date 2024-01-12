// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::{Mutex, Arc};
use tauri::Manager;
use usopp::window::WindowManager;

mod command;
mod init;
mod tray;
mod event;
mod window;


fn main() {
    init::init_app();
    tauri::Builder::default()
        .setup(move |app| {
            let win_manager_state: Arc<Mutex<WindowManager>> = Arc::new(Mutex::new(WindowManager::new(app.handle())));
        
            if let Some(main_win) = app.app_handle().get_window("usopp") {
                win_manager_state.lock().unwrap().set_window("usopp", main_win);
            }
            app.manage(win_manager_state);
            Ok(())
        })
        .system_tray(tauri::SystemTray::new().with_menu(tray::tray_menu()))
        .on_system_tray_event(tray::tray_event)
        .invoke_handler(tauri::generate_handler![
            command::search,
            command::open,
            command::window_change,
            command::window_create
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
