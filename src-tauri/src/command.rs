use std::{time::{Instant, Duration}, sync::{Mutex, Arc}};
use tokio::process::Command;

use tauri::{LogicalSize, State, Window};
use usopp::{
    config::MAX_LIST_SIZE,
    dto::{Application, SearchResultPayLoad, StorageData},
    utils::{search_apps, search_folders, get_window_position}, window::{WindowManager, WindowInfo},
};

// 根据输入的字符串搜索应用程序
// 暂时不考虑中文搜索、MacOs及Linux
#[tauri::command]
pub fn search(name: &str) -> Result<StorageData, Vec<Application>> {
    let start = Instant::now();
    let mut result: Vec<SearchResultPayLoad> = Vec::new();
    // 获取应用程序
    let apps = search_apps(name);
    let folders = search_folders(name);

    if !apps.is_empty() {
        result.extend(apps);
    }

    if !folders.is_empty() {
        result.extend(folders);
    }

    let limited_result = result
        .into_iter()
        .take(MAX_LIST_SIZE)
        .collect::<Vec<SearchResultPayLoad>>();
    let elapsed = Instant::now().duration_since(start);
    println!("search time: {} ms", elapsed.as_millis());
    Ok(StorageData {
        data: serde_json::to_value(limited_result).unwrap(),
        status: true,
    })
}

#[tauri::command]
pub fn open(_window: Window, r_type: &str, path: &str, directive: &str) {
    match r_type {
        "Application" => {
            Command::new(path)
                .spawn()
                .expect("Failed to open application");
        }
        "Folder" => {
            if directive.is_empty() {
                let mut command = Command::new("explorer");
                command.arg(path);
                command.spawn().expect("Failed to open folder");
            }
            if directive == "vscode" {
                let mut cmd = Command::new("cmd");
                cmd.args(&["/C", "code", path]);

                let _ = cmd.spawn().map_err(|e| e.to_string());
            }
            if directive == "idea" {
                let apps = search("idea");
                match apps {
                    Ok(res) => {
                        if res.status {
                            let result: serde_json::Value = serde_json::json!(res.data);
                            let first_element = result.get(0);
                            match first_element {
                                Some(element) => {
                                    if let Some(r_main_pro_path) = element.get("r_main_pro_path") {
                                        if let Some(r_main_pro_path_str) = r_main_pro_path.as_str()
                                        {
                                            let idea_path = format!("{}/bin/", r_main_pro_path_str);
                                            let mut cmd = Command::new("cmd");
                                            cmd.args(&["/C", "idea.bat", path])
                                                .current_dir(idea_path);

                                            let _ = cmd.spawn().map_err(|e| e.to_string());
                                        }
                                    }
                                }
                                None => {
                                    println!("没有获取到idea app");
                                }
                            }
                        }
                    }
                    Err(_err) => {}
                }
            }
        }

        _ => {}
    }
}

#[tauri::command]
pub fn window_change(_window: Window, event: String, win_manager: State<'_, Arc<Mutex<WindowManager>>>) -> Result<(), String> {
    // 在异步上下文中获取锁
    let mut manager = win_manager.lock().unwrap();
    match event.as_str() {
        "blur" => {
            manager.hide_all_window();
            Ok(())
        },
        "focus" => {
            // window.show().unwrap();
            manager.show_all_window();
            Ok(())
        },
        "close_child_win" => {
            manager.close_child_window();
            Ok(())
        }
        _ => {
            Ok(())
        }
    }
}

#[tauri::command]
pub async fn window_create<'a>(_window: Window, label: &'a str, win_manager: State<'a, Arc<Mutex<WindowManager>>>) -> Result<(), String> { 
    let mut manager = win_manager.lock().unwrap();
  
    if let Some(main_window) = manager.get_window("usopp") {
     
        let main_window = main_window.clone();
        let position = get_window_position(&main_window);
        manager.create_window(label, "https://www.baidu.com", position.x.into(), position.y as f64);
    }
    Ok(())
}

#[tauri::command]
pub fn window_resize(_window: Window, width: i32, height: i32, w_type: &str, win_manager: State<Arc<Mutex<WindowManager>>>) {
    match w_type {
        // 主窗口变化
        "window" => {
            let manager = win_manager.lock().unwrap();
            if let Some(win) = manager.get_window("usopp") {
                let _ = win.set_size(LogicalSize { width, height });
            }
            
        }
        // 主窗口内嵌webview
        "webview" => {
            let manager = win_manager.lock().unwrap();
            manager.update_window_size(width, height);
        }
        _ => {}
    }
    
   
}

#[tauri::command]
pub fn set_parent_window_info(_window: Window, width: f64, height: f64, top: f64, win_manager: State<'_, Arc<Mutex<WindowManager>>>) -> Result<(), String> {
    
    let mut manager = win_manager.lock().unwrap();
    manager.set_window_info(WindowInfo {
        width,
        height,
        top
    });
  Ok(())
}
