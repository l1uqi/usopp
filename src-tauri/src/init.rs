use usopp::{dto::Application, storage::{write_data}};
use winreg::{RegKey, enums::{HKEY_LOCAL_MACHINE, HKEY_CURRENT_USER, KEY_READ}};

const UNINSTALL_KEY: &str = "Software\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall";

const UNINSTALL_KEY_2: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall";

pub fn init_app() {
  let apps: Vec<Application> = get_application();
  write_data("apps", serde_json::json!(apps));
}

fn get_subkeys(reg_key: &RegKey) -> Vec<String> {
  reg_key.enum_keys().map(|x| x.unwrap()).collect()
}

// 通过exe路径获取icon路径
// 调用windows api ExtractAssociatedIconW获取icon路径
fn get_icon_for_exe(exe_path: &str) {
}

// 通过icon路径获取exe路径
fn fix_icon_path(display_icon: &str) -> String {
  if let Some(extension) = std::path::Path::new(display_icon).extension() {
    let icon_path_without_extension = display_icon.trim_end_matches(extension.to_str().unwrap());
    let icon_path_with_exe = format!("{}exe", icon_path_without_extension);
    return icon_path_with_exe;
  }
  display_icon.to_owned()
}

fn get_application_info(reg_key: &RegKey, subkey: &str) -> Option<Application> {
  let app_key = reg_key.open_subkey_with_flags(subkey, KEY_READ).ok()?;
  let display_name: String = app_key.get_value("DisplayName").unwrap_or_default();

  if !display_name.is_empty() {
      let display_icon: String = app_key.get_value("DisplayIcon").unwrap_or_default();
      let mut run_path: String = String::new();
      if !display_icon.is_empty(){
        run_path = fix_icon_path(&display_icon);
        get_icon_for_exe(&run_path);
        println!("run_path: {}", run_path)
      }
      let install_location: String = app_key.get_value("InstallLocation").unwrap_or_default();

      Some(Application {
          name: subkey.to_owned(),
          display_name,
          icon_path: display_icon,
          install_location,
          run_path
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
