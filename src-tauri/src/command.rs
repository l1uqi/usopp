use std::time::Instant;
use tokio::process::Command;

use tauri::Window;
use usopp::{
    config::MAX_LIST_SIZE,
    dto::{Application, SearchResultPayLoad, StorageData},
    utils::{search_apps, search_folders},
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
pub fn open(window: Window, r_type: &str, path: &str, directive: &str) {
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
                // let status = Command::new("cmd")
                //     .args(&["/C", "code", path])
                //     .status()
                //     .expect("Failed to open folder with VSCode");
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
                                            // let status = Command::new("cmd")
                                            //     .args(&["/c", "idea.bat", path])
                                            //     .current_dir(idea_path)
                                            //     .status()
                                            //     .await()
                                            //     .expect("Failed to open folder with VSCode");

                                            // if !status.success() {
                                            //     panic!("Failed to open folder with VSCode");
                                            // }
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
                // let status = Command::new("cmd")
                // .args(&["/C", "idea", path])
                // .status()
                // .expect("Failed to open folder with VSCode");

                // if !status.success() {
                //     panic!("Failed to open folder with VSCode");
                // }
            }
        }

        _ => {}
    }
}

#[tauri::command]
pub fn window_change(window: Window, event: String) {
    match event.as_str() {
        "blur" => window.hide().unwrap(),
        "focus" => {
            window.show().unwrap();
            let _ = window.set_focus();
        }
        _ => {}
    }
}
