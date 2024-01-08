use serde::{Serialize, Deserialize};

use crate::enums::SearchPayLoadEvent;

#[derive(Serialize, Deserialize, Debug)]
pub struct Application {
  pub name: String,
  pub soft_name: String, // 软件拼音名
  pub soft_publisher: String, // 软件发布者
  pub soft_version: String, // 软件版本
  pub soft_main_pro_path: String, // 软件主程序路径
  pub soft_uninstall_path: String, // 软件卸载路径
  pub soft_size: u64,
}

#[derive(serde::Serialize)]
pub struct SearchResultPayLoad {
  pub name: String,
  pub text_name: String,
  pub r_type: String,
  pub r_publisher: Option<String>,
  pub r_version: Option<String>, 
  pub r_exe_path: Option<String>,
  pub r_icon_path: Option<String>,

  // pub soft_icon_buffer: Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FolderInfo {
  pub name: String,
  pub path: String,
}

#[derive(Serialize, Debug)]
pub struct StorageData {
    pub data: serde_json::Value,
    pub status: bool,
}

#[derive(Clone, Serialize)]
struct Payload {
    status: String
}
