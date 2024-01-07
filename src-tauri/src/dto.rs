#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Application {
  pub name: String,
  pub soft_name_init:String, // 软件名
  pub soft_name: String, // 软件拼音名
  pub soft_publisher: String, // 软件发布者
  pub soft_version: String, // 软件版本
  pub soft_main_pro_path: String, // 软件主程序路径
  pub soft_uninstall_path: String, // 软件卸载路径
  pub soft_size: u64,
}

#[derive(serde::Serialize, Debug)]
pub struct ApplicationPayLoad {
  pub name: String,
  pub soft_name_init:String, // 软件名
  pub soft_name: String, // 软件拼音名
  pub soft_publisher: String,
  pub soft_version: String, 
  pub soft_run_path: String,
  pub soft_icon_path: String,
  pub soft_icon_buffer: Vec<u8>
}

#[derive(serde::Serialize, Debug)]
pub struct StorageData {
    pub data: serde_json::Value,
    pub status: bool,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    status: String
}
