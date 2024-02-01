use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{database::IndexDatabase, window::WindowManager};

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

impl<'de> Deserialize<'de> for SearchStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let status_str: &str = Deserialize::deserialize(deserializer)?;
        match status_str {
            "InProgress" => Ok(SearchStatus::InProgress),
            "Completed" => Ok(SearchStatus::Completed),
            "Error" => Ok(SearchStatus::Error),
            _ => Err(serde::de::Error::custom("Invalid SearchStatus value")),
        }
    }
}

#[derive(Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum FileType {
    Application,
    LNK,
    Directory,
    File,
    ZIP,
    TXT,
    PDF,
    JPG,
    GIF,
    PNG,
    XLSX,
    PPT,
    DOC,
    JS,
}

impl FileType {
    pub fn to_string(&self) -> String {
        match self {
            FileType::Application => "Application".to_string(),
            FileType::LNK => "LNK".to_string(),
            FileType::Directory => "Directory".to_string(),
            FileType::File => "File".to_string(),
            FileType::ZIP => "ZIP".to_string(),
            FileType::TXT => "TXT".to_string(),
            FileType::PDF => "PDF".to_string(),
            FileType::JPG => "JPG".to_string(),
            FileType::GIF => "GIF".to_string(),
            FileType::PNG => "PNG".to_string(),
            FileType::XLSX => "XLSX".to_string(),
            FileType::PPT => "PPT".to_string(),
            FileType::DOC => "DOC".to_string(),
            FileType::JS => "JS".to_string(),
        }
    }
}

impl Serialize for FileType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let type_str = match self {
            FileType::Application => "Application",
            FileType::LNK => "LNK",
            FileType::ZIP => "ZIP",
            FileType::TXT => "TXT",
            FileType::JPG => "JPG",
            FileType::PNG => "PNG",
            FileType::GIF => "GIF",
            FileType::Directory => "Directory",
            FileType::File => "File",
            FileType::PDF => "PDF",
            FileType::XLSX => "XLSX",
            FileType::PPT => "PPT",
            FileType::DOC => "DOC",
            FileType::JS => "JS",
        };
        serializer.serialize_str(type_str)
    }
}

// impl<'de> Deserialize<'de> for FileType {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let status_str: &str = Deserialize::deserialize(deserializer)?;
//         match status_str {
//             "Application" => Ok(FileType::Application),
//             "JPG" => Ok(FileType::JPG),
//             "LNK" => Ok(FileType::LNK),
//             "ZIP" => Ok(FileType::ZIP),
//             "TXT" => Ok(FileType::TXT),
//             "PNG" => Ok(FileType::PNG),
//             "GIF" => Ok(FileType::GIF),
//             "Directory" => Ok(FileType::Directory),
//             "File" => Ok(FileType::File),
//             "PDF" => Ok(FileType::PDF),
//             "XLSX" => Ok(FileType::XLSX),
//             "PPT" => Ok(FileType::PPT),
//             "DOC" => Ok(FileType::DOC),
//             "JS" => Ok(FileType::JS),
//             _ => Err(serde::de::Error::custom("Invalid FileType value")),
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Application {
    pub name: String,
    pub soft_name: String, // 软件拼音名
    pub soft_icon_name: String,
    pub soft_publisher: String,      // 软件发布者
    pub soft_version: String,        // 软件版本
    pub soft_main_pro_path: String,  // 软件主程序路径
    pub soft_uninstall_path: String, // 软件卸载路径
    pub soft_size: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PayLoad {
    pub status: SearchStatus,
    pub data: Vec<FileEntry>,
}

#[derive(Serialize, Debug)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub f_type: FileType,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct FileIndex {
    pub entries: Vec<FileEntry>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileEntry {
    pub path: String,
    pub name: String,
    pub size: Option<u64>,
    pub file_type: FileType,
    pub icon_path: Option<String>,
}

pub struct Manage {
    pub win: Arc<Mutex<WindowManager>>,
    pub database: Arc<Mutex<IndexDatabase>>,
}
