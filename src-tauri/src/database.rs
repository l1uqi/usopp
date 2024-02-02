use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use rusqlite::{Connection, OptionalExtension};
use tauri::api::path::local_data_dir;

use crate::dto::FileEntry;

pub struct IndexDatabase {
    pub conn: Connection,
}

impl IndexDatabase {
    pub fn new() -> Self {
        let db_dir: PathBuf = Path::new(&local_data_dir().unwrap()).join("Usopp");
        let db_path = db_dir.join("database.db");
        let db_path = PathBuf::from(db_path);
        fs::create_dir_all(&db_dir).expect("Failed to create database directory");
        println!("db path: {}", db_path.to_string_lossy());
        let conn = Connection::open(db_path).unwrap();

        // 检查表是否存在
        let table_exists: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='file_index'",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0)
            > 0;
        // 如果表不存在，则创建表
        if !table_exists {
            conn.execute(
                "CREATE TABLE file_index (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  path TEXT NOT NULL UNIQUE,
                  name TEXT NOT NULL,
                  size INTEGER,
                  file_type TEXT NOT NULL,
                  icon_path TEXT
              )",
                [],
            )
            .expect("Failed to create file_index table");
        }
        Self { conn }
    }
    pub fn insert(&mut self, files: Vec<FileEntry>) {
        let transaction = self.conn.transaction().unwrap();
        let mut stmt = transaction
            .prepare(
              "INSERT OR REPLACE INTO file_index (path, name, size, file_type, icon_path)
              VALUES (?, ?, ?, ?, ?)"
              ,
            )
            .unwrap();
        for entry in files {
            let size = entry.size.unwrap_or_default();
            let icon_path = entry
                .icon_path
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or_default();
            let file_type = entry.file_type.to_string();

            let _ = stmt.execute([
              &entry.path,
              &entry.name,
              &size.to_string(),
              &file_type,
              icon_path,
          ]);

            
        }
        drop(stmt);
        transaction.commit().unwrap();
    }
    pub fn search_by_name(&self, name: &str) -> Result<Vec<FileEntry>, Box<dyn std::error::Error>> {
        let mut query = String::new();
        if name.len() < 2 {
            query = format!("SELECT * FROM file_index WHERE name LIKE '%{}%' LIMIT 100", name)
        } else {
            query = format!("SELECT * FROM file_index WHERE name LIKE '%{}%'", name)

        }
        
        let mut stmt = self.conn.prepare(&query)?;
        let file_iter = stmt.query_map([], |row| {
            Ok(FileEntry {
                path: row.get(1)?,
                name: row.get(2)?,
                size: row.get(3)?,
                file_type: crate::dto::FileType::Application,
                icon_path: row.get(5)?,
            })
        })?;

        let mut results: Vec<FileEntry> = Vec::new();
        for row in file_iter {
            results.push(row.unwrap());
            // println!("Found person {:?}", row.unwrap());
        }

        Ok(results)
    }
}
