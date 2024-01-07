use usopp::{dto::Application, storage::write_data, utils::get_pin_yin};
use winreg::{RegKey, enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ}};

const UNINSTALL_KEY: &str = "Software\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall";

const UNINSTALL_KEY_2: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall";

pub fn init_app() {
  let apps: Vec<Application> = get_application();
  write_data("apps", serde_json::json!(apps));
}

fn get_subkeys(reg_key: &RegKey) -> Vec<String> {
  reg_key.enum_keys().map(|x| x.unwrap()).collect()
}

// 获取本地应用信息
fn get_application_info(reg_key: &RegKey, subkey: &str) -> Option<Application> {
  let app_key = reg_key.open_subkey_with_flags(subkey, KEY_READ).ok()?;
  let sys_component: String = app_key.get_value("SystemComponent").unwrap_or_default();
  let soft_name_init: String = app_key.get_value("DisplayName").unwrap_or_default();
  let soft_name = get_pin_yin(&soft_name_init);
  let soft_parent_key: String = app_key.get_value("ParentKeyName").unwrap_or_default();
  let soft_parent_display_name: String = app_key.get_value("ParentDisplayName").unwrap_or_default();
 
  if !soft_name.is_empty() && sys_component!= "1" && soft_parent_key.is_empty() && soft_parent_display_name.is_empty() {
    let soft_publisher: String = app_key.get_value("Publisher").unwrap_or_default();
    let soft_version: String = app_key.get_value("DisplayVersion").unwrap_or_default();
    let soft_main_pro_path: String = app_key.get_value("InstallLocation").unwrap_or_default();
    let soft_uninstall_path: String = app_key.get_value("UninstallString").unwrap_or_default();
    let soft_size: u64 = app_key.get_value("EstimatedSize").unwrap_or_default();

    Some(Application {
        name: subkey.to_owned(),
        soft_name,
        soft_publisher,
        soft_version,
        soft_main_pro_path,
        soft_uninstall_path,
        soft_size,
        soft_name_init,
    })
  } else {
    None
  }
}

pub fn get_application() -> Vec<Application> {
  let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
  let hklm_2 = RegKey::predef(HKEY_CURRENT_USER);

  let uninstall_key: RegKey = hklm.open_subkey_with_flags(UNINSTALL_KEY, KEY_READ).unwrap();
  let uninstall_key_2: RegKey = hklm.open_subkey_with_flags(UNINSTALL_KEY_2, KEY_READ).unwrap();
  let uninstall_key_3: RegKey = hklm_2.open_subkey_with_flags(UNINSTALL_KEY_2, KEY_READ).unwrap();

  let subkeys = get_subkeys(&uninstall_key);
  let subkeys_2 = get_subkeys(&uninstall_key_2);
  let subkeys_3: Vec<String> = get_subkeys(&uninstall_key_3);

  let mut applications: Vec<Application> = vec![];

  for subkey in subkeys {
      if let Some(application) = get_application_info(&uninstall_key, &subkey) {
        applications.push(application);
      }
  }

  for subkey in subkeys_2 {
    if let Some(application) = get_application_info(&uninstall_key_2, &subkey) {
      applications.push(application);
    }
  }

  for subkey in subkeys_3 {
    if let Some(application) = get_application_info(&uninstall_key_3, &subkey) {
      applications.push(application);
    }
  }
  applications
}
