use serde::{Deserialize, Serialize, Serializer};

#[derive(Clone)]
pub enum SearchStatus {
  InProgress,
  Completed,
  Error,
}

impl Serialize for SearchStatus {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
      let status_str = match self {
          SearchStatus::InProgress => "InProgress",
          SearchStatus::Completed => "Completed",
          SearchStatus::Error => "Error",
      };
      serializer.serialize_str(status_str)
  }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Application {
  pub name: String,
  pub soft_name: String, // 软件拼音名
  pub soft_icon_name: String,
  pub soft_publisher: String, // 软件发布者
  pub soft_version: String, // 软件版本
  pub soft_main_pro_path: String, // 软件主程序路径
  pub soft_uninstall_path: String, // 软件卸载路径
  pub soft_size: u64,
}

#[derive(serde::Serialize, Clone)]
pub struct SearchResultPayLoad {
  pub status: SearchStatus,
  pub data: Vec<SearchResult>
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct SearchResult {
  pub name: String,
  pub text_name: String,
  pub r_type: String,
  pub r_publisher: Option<String>,
  pub r_version: Option<String>, 
  pub r_exe_path: Option<String>,
  pub r_icon_path: Option<String>,
  pub r_main_pro_path: Option<String>,

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

#[derive(Serialize, Debug)]
pub struct SystemInfo {
    pub cpu_usage: f32,
    pub memory_usage: f64,
}

#[derive(Clone, Serialize)]
struct Payload {
    status: String
}
