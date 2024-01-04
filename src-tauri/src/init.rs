use usopp::{dto::Application, storage::write_data, icons::{get_exe_path_for_icon, get_icon, get_icon_bigmap, get_bitmap_buffer, save_icon_file}};
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
  let display_name: String = app_key.get_value("DisplayName").unwrap_or_default();

  if !display_name.is_empty() {
      let display_icon: String = app_key.get_value("DisplayIcon").unwrap_or_default();
      let mut run_path: String = String::new();
      let mut icon_path = String::new();
      let mut icon_buffer = vec![];
      if !display_icon.is_empty(){
     
        run_path = get_exe_path_for_icon(&display_icon);
        
        if display_name == "微信" {
          println!("display_icon: {}", display_icon);
          println!("run_path: {}", run_path);
        }
        if let Some(icon) = get_icon(&run_path) {
          if let Some(map) = get_icon_bigmap(icon) {
            if let Some(buffer) = get_bitmap_buffer(map) {
              icon_path = save_icon_file(&buffer, &display_name);
              icon_buffer = buffer;
            }
          }
        }
      }
      let install_location: String = app_key.get_value("InstallLocation").unwrap_or_default();

      Some(Application {
          name: subkey.to_owned(),
          display_name,
          install_location,
          run_path,
          icon_path,
          icon_buffer
      })
  } else {
      None
  }
}

pub fn get_application() -> Vec<Application> {
  let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
  let hklm_2 = RegKey::predef(HKEY_CURRENT_USER);

  let uninstall_key: RegKey = hklm.open_subkey_with_flags(UNINSTALL_KEY, KEY_READ).unwrap();
  let uninstall_key_2: RegKey = hklm_2.open_subkey_with_flags(UNINSTALL_KEY_2, KEY_READ).unwrap();

  let subkeys = get_subkeys(&uninstall_key);
  let subkeys_2 = get_subkeys(&uninstall_key_2);

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
  applications
}
