use std::{
    sync::{Arc, Mutex},
    time::Instant,
};
use strsim::levenshtein;
use tokio::process::Command;

use tauri::{LogicalSize, State, Window};
use usopp::{
    config::MAX_LIST_SIZE, dto::{Application, SearchResult, SearchResultPayLoad, SearchStatus, StorageData}, search::{application::search_applications_by_name, folder::search_folders_by_name}, utils::{get_logical_drive_letters, get_window_position}, window::{WindowInfo, WindowManager}
};


// 多线程搜索
#[tauri::command]
pub async fn async_search(window: Window, name: &str) -> Result<(), String> {
    let start = Instant::now();
    let mut result: Vec<SearchResult> = Vec::new();
    // 获取应用程序
    let apps: Vec<SearchResult> = search_applications_by_name(name);

    if !apps.is_empty() {
        let _ = window.emit("search-result", SearchResultPayLoad {
           data: apps.clone(),
           status: SearchStatus::InProgress
        });
        result.extend(apps);
    }

    // 获取磁盘
    let drives = get_logical_drive_letters();

    // 获取文件夹列表
    let folder_list = search_folders_by_name(&window, &name, drives);

    if !folder_list.is_empty() {
        result.extend(folder_list);
    }

    let elapsed = Instant::now().duration_since(start);
    println!("搜索总耗时{}ms", elapsed.as_millis());

    // 根据 Levenshtein 距离排序结果
    let mut sorted_results: Vec<(SearchResult, usize)> = result.into_iter()
    .map(|result| (result.clone(), levenshtein(&result.name, name)))
    .collect();

    sorted_results.sort_by(|(result1, dist1), (result2, dist2)| {
        match (result1.r_type.as_str(), result2.r_type.as_str()) {
            ("Application", "Application") => dist1.cmp(dist2),
            ("Application", _) => std::cmp::Ordering::Less,
            (_, "Application") => std::cmp::Ordering::Greater,
            _ => dist1.cmp(dist2),
        }
    });
    

    // 提取匹配度最高的前 50 条结果
    let top_results: Vec<SearchResult> = sorted_results.into_iter()
    .take(MAX_LIST_SIZE)
    .map(|(result, _)| result)
    .collect();

    let _ = window.emit("search-result", SearchResultPayLoad {
        data: top_results,
        status: SearchStatus::Completed
    });
     Ok(())
    // Ok(StorageData {
    //     data: serde_json::to_value(limited_result).unwrap(),
    //     status: true,
    // })
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

            }
        }

        _ => {}
    }
}

#[tauri::command]
pub fn window_change(
    _window: Window,
    event: String,
    win_manager: State<'_, Arc<Mutex<WindowManager>>>,
) -> Result<(), String> {
    // 在异步上下文中获取锁
    let mut manager = win_manager.lock().unwrap();
    match event.as_str() {
        "blur" => {
            manager.hide_all_window();
            Ok(())
        }
        "focus" => {
            // window.show().unwrap();
            manager.show_all_window();
            Ok(())
        }
        "close_child_win" => {
            manager.close_child_window();
            Ok(())
        }
        _ => Ok(()),
    }
}

#[tauri::command]
pub async fn window_create<'a>(
    _window: Window,
    label: &'a str,
    win_manager: State<'a, Arc<Mutex<WindowManager>>>,
) -> Result<(), String> {
    let mut manager = win_manager.lock().unwrap();

    if let Some(main_window) = manager.get_window("usopp") {
        let main_window = main_window.clone();
        let position = get_window_position(&main_window);
        manager.create_window(
            label,
            "https://www.baidu.com",
            position.x.into(),
            position.y as f64,
        );
    }
    Ok(())
}

#[tauri::command]
pub fn window_resize(
    _window: Window,
    width: i32,
    height: i32,
    w_type: &str,
    win_manager: State<Arc<Mutex<WindowManager>>>,
) {
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
pub fn set_parent_window_info(
    _window: Window,
    width: f64,
    height: f64,
    top: f64,
    win_manager: State<'_, Arc<Mutex<WindowManager>>>,
) -> Result<(), String> {
    let mut manager = win_manager.lock().unwrap();
    manager.set_window_info(WindowInfo { width, height, top });
    Ok(())
}
