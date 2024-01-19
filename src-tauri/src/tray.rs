use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayEvent, Manager, SystemTrayMenuItem, AppHandle, api};


pub fn tray_menu() -> SystemTrayMenu {
  let open = CustomMenuItem::new("open_window".to_string(), "打开面板");
  let hide = CustomMenuItem::new("hide".to_string(), "隐藏应用");
  let restart = CustomMenuItem::new("restart_app".to_string(), "重启应用");
  let update = CustomMenuItem::new("restart_app".to_string(), "获取更新");
  let quit= CustomMenuItem::new("quit".to_string(), "退出").accelerator("CmdOrControl+Q");

  SystemTrayMenu::new()
  .add_item(open)
  .add_item(hide)
  .add_native_item(SystemTrayMenuItem::Separator)
  .add_item(restart)
  .add_item(update)
  .add_native_item(SystemTrayMenuItem::Separator)
  .add_item(quit)
}

pub fn tray_event(app_handle: &AppHandle, event: SystemTrayEvent) {
  match event {
    SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        "open_window" => {
          let window = app_handle.get_window("main").unwrap();
          window.unminimize().unwrap();
          window.show().unwrap();
        },
        "hide" => {
          let window = app_handle.get_window("main").unwrap();
          window.hide().unwrap();
        }
        "restart_app" => {
          api::process::restart(&app_handle.env());
        }
        "update" => {

        }
        "quit" => {
          std::process::exit(0);
        }
        _ => {}
    },
    #[cfg(target_os = "windows")]
    SystemTrayEvent::LeftClick { .. } => {
      let window = app_handle.get_window("usopp").unwrap();
      window.unminimize().unwrap();
      window.show().unwrap();
      window.set_focus().unwrap();
    }
    _ => {}
}
}
