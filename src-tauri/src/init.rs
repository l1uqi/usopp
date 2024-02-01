use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use usopp::{
    config::EXCLUDED_FOLDERS,
    database::IndexDatabase,
    dto::{FileEntry, FileType},
    utils::get_logical_drive_letters,
};

pub async fn init_app(database: Arc<Mutex<IndexDatabase>>) {
    set_file_index(database);
}

fn set_file_index(database: Arc<Mutex<IndexDatabase>>) {
    let drives = get_logical_drive_letters();
    let mut handles = Vec::new();
    for drie in drives {
        let database_clone = database.clone();
        let handle = thread::spawn(move || {
            let start = Instant::now();
            let path = format!("{}:\\", drie);
            let (list, _size) = index_directory(&PathBuf::from(&path));

            let elapsed = Instant::now().duration_since(start);
            println!(
                "磁盘:{}, 找到条数{:?}, 耗时{}ms",
                drie,
                &list.len(),
                elapsed.as_millis()
            );
            database_clone.lock().unwrap().insert(list)
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn index_directory(directory_path: &Path) -> (Vec<FileEntry>, u64) {
    let mut file_index_list: Vec<FileEntry> = Vec::new();
    let mut total_size: u64 = 0;
    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                // 跳过隐藏目录
                if path.is_dir() {
                    if name.starts_with(".")
                        || name.starts_with("$")
                        || EXCLUDED_FOLDERS.contains(&name.as_str())
                    {
                        continue;
                    }
                }
                let file_type = if path.is_dir() {
                    FileType::Directory
                } else {
                    FileType::File
                };

                let size = if file_type == FileType::File {
                    let metadata = fs::metadata(&path).ok().map(|meta| meta.len());
                    let size = metadata.unwrap();
                    total_size += size;
                    Some(size)
                } else {
                    None
                };

                let mut file_index = FileEntry {
                    name,
                    size,
                    path: path.to_string_lossy().to_string(),
                    file_type,
                    icon_path: None,
                };

                if file_type == FileType::Directory {
                    let (subdirectory_index_list, size) = index_directory(&path);
                    total_size += size;
                    file_index.size = Some(size);
                    file_index_list.extend(subdirectory_index_list);
                }
                file_index_list.push(file_index);
            }
        }
    }
    (file_index_list, total_size)
}
