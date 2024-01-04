#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Application {
  pub name: String,
  pub display_name: String,
  pub run_path: String,
  pub install_location: String,
  pub icon_buffer: Vec<u8>,
  pub icon_path: String,
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
