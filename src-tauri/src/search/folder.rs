use std::{
    fs,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use tauri::Window;

use crate::{
    config::{EXCLUDED_FOLDERS, MAX_DEPTH},
    dto::{FolderInfo, SearchResult, SearchResultPayLoad, SearchStatus},
    icons::{get_bitmap_buffer, get_icon, get_icon_bigmap, save_icon_file},
};

pub fn search_folders_by_name(window: &Window, name: &str, dries: Vec<char>) -> Vec<SearchResult> {
    // let dries = get_logical_drive_letters();
    let folders: Arc<Mutex<Vec<SearchResult>>> = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for drie in dries {
        let name_clone = name.to_owned();
        let window_clone = window.clone();
        let results_clone = Arc::clone(&folders);
        let handle = thread::spawn(move || {
            let start = Instant::now();
            let folders = get_folders_by_drive(drie, MAX_DEPTH);
            let folders = filter_folder_by_name(folders, &name_clone);
            let elapsed = Instant::now().duration_since(start);
            println!(
                "磁盘:{}, 遍历文件深度:{} 找到条数{:?}, 耗时{}ms",
                drie,
                MAX_DEPTH,
                &folders.len(),
                elapsed.as_millis()
            );
            let mut results = results_clone.lock().unwrap();
            let _ = window_clone.emit(
                "search-result",
                SearchResultPayLoad {
                    data: folders.clone(),
                    status: SearchStatus::InProgress,
                },
            );
            results.extend(folders);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let folders_result = folders.lock().unwrap();

    folders_result.clone()
}

// 通过路径获取文件夹信息
fn get_folders_by_drive(letter: char, max_depth: u32) -> Vec<FolderInfo> {
    let mut folder_list = Vec::new();
    let drive_path = format!("{}:\\", letter);
    folder_list.extend(get_folder_info(drive_path, max_depth));
    folder_list
}

fn get_folder_info(folder_path: String, max_depth: u32) -> Vec<FolderInfo> {
    let mut folder_list: Vec<FolderInfo> = Vec::new();

    if let Ok(entries) = fs::read_dir(&folder_path) {
        let mut thread_handles = Vec::new();

        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let folder_name = entry.file_name().to_string_lossy().to_string();
                        let folder_path = entry.path();
                        // 跳过隐藏目录
                        if folder_name.starts_with(".")
                            || EXCLUDED_FOLDERS.contains(&folder_name.as_str())
                        {
                            continue;
                        }

                        let thread_handle = thread::spawn(move || {
                            get_folder_info_recursive(
                                folder_path.to_string_lossy().to_string(),
                                max_depth,
                                1,
                            )
                        });

                        thread_handles.push(thread_handle);
                    }
                }
            }
        }

        println!("磁盘: {}, 开启线程数: {}", folder_path, thread_handles.len());

        for handle in thread_handles {
            if let Ok(subdirectories) = handle.join() {
                folder_list.extend(subdirectories);
            }
        }
    }

    folder_list
}

// 获取目录下文件夹信息 folder_path 目录 max_depth 目录深度 current_depth 当前目录深度
fn get_folder_info_recursive(
    folder_path: String,
    max_depth: u32,
    current_depth: u32,
) -> Vec<FolderInfo> {
    let mut folder_list: Vec<FolderInfo> = Vec::new();

    if current_depth > max_depth && max_depth != 0 {
        return folder_list;
    }

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let folder_name = entry.file_name().to_string_lossy().to_string();
                        let folder_path = entry.path();
                        // 跳过隐藏目录
                        if folder_name.starts_with(".")
                            || EXCLUDED_FOLDERS.contains(&folder_name.as_str())
                        {
                            continue;
                        }
                        let folder_info = FolderInfo {
                            name: folder_name.clone(),
                            path: folder_path.clone().to_string_lossy().to_string(),
                        };
                        folder_list.push(folder_info);

                        let subdirectories = get_folder_info_recursive(
                            folder_path.to_string_lossy().to_string(),
                            max_depth,
                            current_depth + 1,
                        );
                        folder_list.extend(subdirectories);
                    }
                }
            }
        }
    }
    folder_list
}

fn filter_folder_by_name(folder_list: Vec<FolderInfo>, name: &str) -> Vec<SearchResult> {
    let filtered_folders: Vec<FolderInfo> = folder_list
        .into_iter()
        .filter(|folder| folder.name.to_lowercase().replace(" ", "").contains(&name))
        .collect();

    let mut folder_icon_path = String::new();

    if !filtered_folders.is_empty() {
        let folder_path = filtered_folders[0].path.clone();
        if let Some(icon) = get_icon(&folder_path) {
            if let Some(map) = get_icon_bigmap(icon) {
                if let Some(buffer) = get_bitmap_buffer(map) {
                    folder_icon_path = save_icon_file(&buffer, "folder");
                }
            }
        }
    }

    let mut folder_payload: Vec<SearchResult> = vec![];
    filtered_folders.iter().for_each(|item| {
        folder_payload.push(SearchResult {
            name: item.name.clone(),
            text_name: item.name.clone(),
            // r_type: SearchPayLoadEvent::Folder,
            r_type: "Folder".to_string(),
            r_publisher: None,
            r_version: None,
            r_main_pro_path: None,
            r_exe_path: Some(item.path.clone()),
            r_icon_path: Some(folder_icon_path.clone()),
        })
    });

    folder_payload
}
