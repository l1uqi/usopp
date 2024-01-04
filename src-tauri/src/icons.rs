use std::{mem, path::{Path, PathBuf}};
use tauri::api::path::local_data_dir;
use winapi::{um::{ shellapi, wingdi::{ BITMAP, GetObjectW}, winuser::{ICONINFO, GetIconInfo}}, shared::windef::{HICON, HBITMAP}};

// 通过exe路径获取icon句柄
// https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetfileinfow
pub fn get_icon(exe_path: &str) -> Option<HICON> {
  let wide_path: Vec<u16> = exe_path.encode_utf16().chain(std::iter::once(0)).collect();
  let wide_path_ptr = wide_path.as_ptr();
  unsafe {
    let mut info: shellapi::SHFILEINFOW = std::mem::zeroed();
    let result = shellapi::SHGetFileInfoW(
        wide_path_ptr,
        0,
        &mut info as *mut shellapi::SHFILEINFOW,
        std::mem::size_of::<shellapi::SHFILEINFOW>() as u32,
        shellapi::SHGFI_ICON | shellapi::SHGFI_LARGEICON,
    );
    if result != 0 {
      return Some(info.hIcon);
    }
  }
  None
}

// 获取位图句柄
// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-geticoninfo
pub fn get_icon_bigmap(hicon: HICON) -> Option<HBITMAP> {
  let mut icon_info = unsafe { mem::zeroed::<ICONINFO>() };
  if unsafe { GetIconInfo(hicon, &mut icon_info as *mut _) } == 0 {
      return None;
  }

  if !icon_info.hbmColor.is_null() {
    Some(icon_info.hbmColor)
  } else if !icon_info.hbmMask.is_null() {
      Some(icon_info.hbmMask)
  } else {
      None
  }
}

// 获取位图数据
// https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getbitmapbits
pub fn get_bitmap_buffer(bitmap: HBITMAP) -> Option<Vec<u8>> {
  let mut bitmap_info: BITMAP = unsafe { std::mem::zeroed() };
  
  if unsafe { GetObjectW(bitmap as _, std::mem::size_of::<BITMAP>() as i32, &mut bitmap_info as *mut _ as *mut _) } == 0 {
      println!("none{:?}", bitmap);
      return None;
  }

  let bitmap_size = (bitmap_info.bmWidth * bitmap_info.bmHeight * (bitmap_info.bmBitsPixel / 8) as i32) as usize;

  let mut buffer: Vec<u8> = vec![0; bitmap_size];

  // 将位图数据复制到内存中
  let result = unsafe {
      winapi::um::wingdi::GetBitmapBits(
          bitmap,
          bitmap_size as i32,
          buffer.as_mut_ptr() as *mut std::ffi::c_void,
      )
  };

  if result != 0 {
      Some(buffer)
  } else {
      None
  }
}

// 保存icon到本地
pub fn save_icon_file(buffer: &Vec<u8>, file_name: &str) -> String {
  let width = 32;
  let height = 32;
  let output_directory = Path::new(&local_data_dir().unwrap()).join("Usopp").join("icons");
  let mut bmp_file_path =  PathBuf::from(output_directory);
  bmp_file_path.push(format!("{name}.png", name = file_name));
  let img = image::ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
    let pixel_index = ((y * width + x) * 4) as usize;
      image::Rgba([buffer[pixel_index + 2], buffer[pixel_index + 1], buffer[pixel_index], buffer[pixel_index + 3]])
  });
  let _ =img.save(bmp_file_path.clone());
  bmp_file_path.to_str().unwrap_or("invalid_path").to_string()
}

// 通过icon路径获取exe路径
pub fn get_exe_path_for_icon(display_icon: &str) -> String {
  if let Some(extension) = std::path::Path::new(display_icon).extension() {
    let icon_path_without_extension = display_icon.trim_end_matches(extension.to_str().unwrap());
    let icon_path_with_exe = format!("{}exe", icon_path_without_extension);
    return icon_path_with_exe.trim_matches('"').to_owned();
  }
  display_icon.to_owned()
}
