use std::{
    fs,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use tauri::Window;

use crate::{
    config::{EXCLUDED_FOLDERS, MAX_DEPTH},
    dto::{FileType, FileEntry, PayLoad, SearchStatus},
};

pub fn search_files_by_name(window: &Window, name: &str, dries: Vec<char>) -> Vec<FileEntry> {
    // let dries = get_logical_drive_letters();
    let folders: Arc<Mutex<Vec<FileEntry>>> = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for drie in dries {
        let name_clone = name.to_owned();
        let window_clone = window.clone();
        let results_clone: Arc<Mutex<Vec<FileEntry>>> = Arc::clone(&folders);
        let handle = thread::spawn(move || {
            let start = Instant::now();
            // let file_list = get_file_list_by_name(&name_clone, drie, MAX_DEPTH);
            let mut file_list = Vec::new();
            file_list.extend(get_file_list_info(
                &window_clone,
                &name_clone,
                format!("{}:\\", drie),
                MAX_DEPTH,
            ));
            let file_list = file_info_to_result_info(file_list, &name_clone);
            let elapsed = Instant::now().duration_since(start);
            println!(
                "磁盘:{}, 找到条数{:?}, 耗时{}ms",
                drie,
                &file_list.len(),
                elapsed.as_millis()
            );
            let mut results = results_clone.lock().unwrap();
            results.extend(file_list);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let folders_result = folders.lock().unwrap();

    folders_result.clone()
}

// 通过路径获取文件信息
// fn get_file_list_by_name(name: &str, letter: char, max_depth: u32) -> Vec<FileInfo> {
//     let mut files = Vec::new();
//     let drive_path = format!("{}:\\", letter);
//     files.extend(get_file_list_info(name, drive_path, max_depth));
//     files
// }

fn get_file_list_info(
    window: &Window,
    name: &str,
    file_path: String,
    max_depth: u32,
) -> Vec<FileEntry> {
    let mut files: Vec<FileEntry> = Vec::new();

    if let Ok(entries) = fs::read_dir(&file_path) {
        let mut thread_handles = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    let f_name = entry.file_name().to_string_lossy().to_string();
                    let f_path: std::path::PathBuf = entry.path();
                    if file_type.is_dir() {
                        // 跳过隐藏目录
                        if f_name.starts_with(".") || EXCLUDED_FOLDERS.contains(&f_name.as_str()) {
                            continue;
                        }
                        let name_clone = name.to_owned();
                        let window_clone = window.clone();

                        let thread_handle = thread::spawn(move || {
                            let start_time = Instant::now();
                            let result = get_file_list_info_recursive(
                                name_clone.clone(),
                                f_path.to_string_lossy().to_string(),
                                max_depth,
                                1,
                            );
                            let duration = start_time.elapsed();

                            let result: Vec<FileEntry> = result
                                .into_iter()
                                .filter(|file: &FileEntry| {
                                    file.name
                                        .to_lowercase()
                                        .replace(" ", "")
                                        .contains(&name_clone)
                                })
                                .collect();
                            // println!(
                            //     "线程执行完成，目录: {}, 执行时间: {:?}, 匹配数: {}",
                            //     f_path.to_string_lossy(),
                            //     duration,
                            //     result.len()
                            // );
                            let _ = window_clone.emit(
                                "search-result",
                                PayLoad {
                                    data: result.clone(),
                                    status: SearchStatus::InProgress,
                                },
                            );
                            result
                        });
                        thread_handles.push(thread_handle);
                    }
                }
            }
        }

        for handle in thread_handles {
            if let Ok(subdirectories) = handle.join() {
                files.extend(subdirectories);
            }
        }
    }

    files
}

// 获取目录下文件信息 file_path 目录 max_depth 目录深度 current_depth 当前目录深度
fn get_file_list_info_recursive(
    name: String,
    file_path: String,
    max_depth: u32,
    current_depth: u32,
) -> Vec<FileEntry> {
    let mut file_list: Vec<FileEntry> = Vec::new();

    if current_depth > max_depth && max_depth != 0 {
        return file_list;
    }

    if let Ok(entries) = fs::read_dir(file_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    let f_name = entry.file_name().to_string_lossy().to_string();
                    let f_path = entry.path();
                    let path: String = f_path.to_string_lossy().to_string();
                    if file_type.is_dir() {
                        // 跳过隐藏目录
                        if f_name.starts_with(".") || f_name.starts_with("$") || EXCLUDED_FOLDERS.contains(&f_name.as_str()) {
                            continue;
                        }

                        let f_info = FileEntry {
                            name: f_name.clone(),
                            size: None,
                            path: path.clone(),
                            icon_path: None,
                            file_type: FileType::Directory,
                        };
                        file_list.push(f_info);

                        let subdirectories = get_file_list_info_recursive(
                            name.clone(),
                            f_path.to_string_lossy().to_string(),
                            max_depth,
                            current_depth + 1,
                        );
                        file_list.extend(subdirectories);
                    } else {
                        if f_name.to_lowercase().replace(" ", "").contains(&name) {
                            let f_info = FileEntry {
                                name: f_name.clone(),
                                file_type: FileType::File,
                                path: path.clone(),
                                icon_path: None,
                                size: None
                            };
                            file_list.push(f_info);
                        }
                    }
                }
            }
        }
    }
    file_list
}

fn file_info_to_result_info(file_list: Vec<FileEntry>, name: &str) -> Vec<FileEntry> {
    // 过滤文件夹
    file_list
        .into_iter()
        .filter(|file| file.name.to_lowercase().replace(" ", "").contains(&name))
        .collect()
}
