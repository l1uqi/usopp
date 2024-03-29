// 应用列表
pub const STORAGE_APPS_KEY: &str = "apps";
// 文件列表
pub const STORAGE_FOLDERS_KEY: &str = "folders";
// 排除文件夹列表
pub const EXCLUDED_FOLDERS: &[&str] = &["node_modules", "Temp", "Windows", "AppData", "SysWOW64", "Microsoft SDKs", "tmp", "dist", "logs", "log", "npm-cache", "cache", "pnpm"];
// 应用列表注册表
pub const UNINSTALL_KEY: &str = "Software\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall";
// 应用列表注册表
pub const UNINSTALL_KEY_2: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall";
// 文件遍历深度
pub const MAX_DEPTH: u32 = 0;
// 列表最大条数
pub const MAX_LIST_SIZE: usize = 30;
