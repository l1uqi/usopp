// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::{Arc, Mutex};
use tauri::{Manager, WindowEvent};
use usopp::window::WindowManager;


mod command;
mod init;
mod tray;
mod window;

fn setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    
    let handle = app.handle();
    let mutex_wins: Arc<Mutex<WindowManager>> =
        Arc::new(Mutex::new(WindowManager::new(handle.clone())));

    if let Some(main_win) = app.app_handle().get_window("usopp") {
        let mutex_wins_clone = mutex_wins.clone();
        // 当前主窗口加入到windows中
        mutex_wins_clone.lock().unwrap().set_window("usopp", main_win.clone());
        main_win.on_window_event(move | event | {
            match event {
                WindowEvent::Moved(position) => {
                    let p = position.clone();
                    mutex_wins_clone.lock().unwrap().update_window_position(p)
                    
                }
                _ => {}
            }
        });
        app.manage(mutex_wins);
    }
    Ok(())
}

fn main() {
    init::init_app();
    tauri::Builder::default()
        .setup(setup)
        .system_tray(tauri::SystemTray::new().with_menu(tray::tray_menu()))
        .on_system_tray_event(tray::tray_event)
        .invoke_handler(tauri::generate_handler![
            command::search,
            command::open,
            command::window_change,
            command::window_create,
            command::window_resize,
            command::set_parent_window_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
