use std::process::Command;

use usopp::{storage::read_data, dto::{Application, StorageData}};

// 根据输入的字符串搜索应用程序
// 暂时不考虑中文搜索、MacOs及Linux
#[tauri::command]
pub fn search(name: &str) -> Result<StorageData ,Vec<Application>> {
    let mut apps: Vec<Application> = vec![];
    let result = read_data("apps");
    match result {
        Ok(res) => {
          apps = res.data.as_array().expect("Invalid data format")
          .iter()
          .map(|app| {
              serde_json::from_value::<Application>(app.clone()).expect("Failed to deserialize")
          })
          .collect();
        }
        Err(e) => {
            println!("{}", e);
        }
    }
   
    let filtered_apps: Vec<&Application> = apps
    .iter()
    .filter(|app| app.display_name.contains(name))
    .collect();
    Ok(StorageData {
        data: serde_json::to_value(filtered_apps).unwrap(),
        status: true,
    })
}

#[tauri::command]
pub fn open(app_path: &str) {
   Command::new(app_path)
        .spawn()
        .expect("Failed to open application");
}
