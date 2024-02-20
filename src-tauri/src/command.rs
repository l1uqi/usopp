
use std::time::Instant;
use tokio::process::Command;

use tauri::{LogicalSize, State, Window};
use usopp::{
    dto::{FileEntry, FileType, Manage, PayLoad, SearchStatus, StorageData},
    search::file::search_files_by_name,
    utils::{get_logical_drive_letters, get_sorted_result, get_window_position, remove_file_index},
    window::WindowInfo,
};

#[tauri::command]
pub fn sorted(data: Vec<FileEntry>, name: &str) -> Result<StorageData, Vec<FileEntry>> {
    let result = get_sorted_result(data, name);
    Ok(StorageData {
        data: serde_json::to_value(result).unwrap(),
        status: true,
    })
}

#[tauri::command]
pub async fn async_search(
    window: Window,
    name: &str,
    manage: State<'_, Manage>,
) -> Result<(), String> {
    let start: Instant = Instant::now();
    let database = manage.database.lock().unwrap();
    let mut result: Vec<FileEntry> = Vec::new();
    if let Ok(data) =  database.search_by_name(name) {
        println!("数据库中找到{}条记录", data.len());
        result.extend(data);
    }

    let mut result = remove_file_index(result, database);

    // let mut result: Vec<FileEntry> = file_index
    //     .clone()
    //     .into_iter()
    //     .filter(|file: &FileEntry| file.name.to_lowercase().replace(" ", "").contains(name))
    //     .collect();

    if result.is_empty() {
        let drives = get_logical_drive_letters();
        let file_list = search_files_by_name(&window, &name, drives);
        if !file_list.is_empty() {
            result.extend(file_list);
        }
    }

    let result = get_sorted_result(result, name);

    let elapsed = Instant::now().duration_since(start);

    println!("搜索总耗时{}ms", elapsed.as_millis());

    let _ = window.emit(
        "search-result",
        PayLoad {
            data: result,
            status: SearchStatus::Completed,
        },
    );
    Ok(())
}

#[tauri::command]
pub fn open(_window: Window, r_type: FileType, path: &str, directive: &str) {
    match r_type {
        FileType::Directory => {
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
            if directive == "idea" {}
        }
        _ => {
            let _ = Command::new("cmd")
                .args(&["/C", "start", path])
                .spawn()
                .map_err(|e| e.to_string());
        }
    }
}

#[tauri::command]
pub fn window_change(
    _window: Window,
    event: String,
    manage: State<'_, Manage>,
) -> Result<(), String> {
    // 在异步上下文中获取锁
    let mut manager = manage.win.lock().unwrap();
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
    manage: State<'_, Manage>,
) -> Result<(), String> {
    let mut manager = manage.win.lock().unwrap();

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
    manage: State<'_, Manage>,
) {
    let manager = manage.win.lock().unwrap();
    match w_type {
        // 主窗口变化
        "window" => {
            if let Some(win) = manager.get_window("usopp") {
                let _ = win.set_size(LogicalSize { width, height });
            }
        }
        // 主窗口内嵌webview
        "webview" => {
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
    manage: State<'_, Manage>,
) -> Result<(), String> {
    let mut manager = manage.win.lock().unwrap();
    manager.set_window_info(WindowInfo { width, height, top });
    Ok(())
}
