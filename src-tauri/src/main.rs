// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::{Mutex, Arc};
use tauri::{Manager, WindowEvent};
use usopp::window::WindowManager;

mod command;
mod init;
mod tray;
mod window;


fn main() {
    init::init_app();
    tauri::Builder::default()
        .setup(move |app| {
            let win_manager_state: Arc<Mutex<WindowManager>> = Arc::new(Mutex::new(WindowManager::new(app.handle())));
        
            if let Some(main_win) = app.app_handle().get_window("usopp") {
                let win_manager_state_clone = win_manager_state.clone();
                win_manager_state.lock().unwrap().set_window("usopp", main_win.clone());
                
                main_win.on_window_event(move | event | {
                    match event {
                        WindowEvent::Moved(position) => {
                            let p = position.clone();
                            win_manager_state_clone.lock().unwrap().update_window_position(p)
                            
                        }
                        _ => {}
                    }
                })
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
            command::window_create,
            command::window_resize,
            command::set_parent_window_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
