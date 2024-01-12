use std::{collections::HashMap, sync::{Arc, Mutex}};
use tauri::{Window, AppHandle, Manager};


pub struct WinManager {
  pub manager: Arc<Mutex<Option<WindowManager>>>,
}


#[derive(Clone)]
pub struct WindowManager {
    windows: HashMap<String, Window>,
    handler: AppHandle,
}

impl WindowManager {
    pub fn new(handler: AppHandle) -> Self {
        Self {
            windows: HashMap::new(),
            handler,
        }
    }

    pub fn create_window(
        &mut self,
        label: &str,
        url: &str,
        width: f64,
        height: f64,
        x: f64,
        y: f64,
    ) {
        if let Ok(window) = tauri::WindowBuilder::new(
            &self.handler,
            label,
            tauri::WindowUrl::External(url.parse().unwrap()),
        )
        .inner_size(width, height)
        .position(x, y)
        .decorations(false)
        .transparent(true)
        .build()
        {
            self.windows.insert(label.to_string(), window);
        }
    }

    pub fn set_window(&mut self, label: &str, window: Window) {
        self.windows.insert(label.to_string(), window);
    }

    pub fn get_window(&self, label: &str) -> Option<&Window> {
        self.windows.get(label)
    }

    pub fn show_window(&self, label: &str) {
        if let Some(window) = self.get_window(label) {
            window.show().expect("Failed to show window");
        }
    }

    pub fn hide_window(&self, label: &str) {
        if let Some(window) = self.get_window(label) {
            window.hide().expect("Failed to hide window");
        }
    }

    pub fn close_window(&mut self, label: &str) {
        if let Some(window) = self.windows.remove(label) {
            window.close().expect("Failed to close window");
        }
    }
}
