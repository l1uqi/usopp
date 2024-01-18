use std::{collections::HashMap, thread, time::Duration};
use tauri::{AppHandle, LogicalSize, Manager, PhysicalPosition, Window};

#[derive(Debug)]
pub struct WindowInfo {
    pub width: f64,
    pub height: f64,
    pub top: f64,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct WindowManager {
    windows: HashMap<String, Window>,
    handler: AppHandle,
    width: f64,
    height: f64,
    top: f64,
}

#[allow(dead_code)]
impl WindowManager {
    pub fn new(handler: AppHandle) -> Self {
        Self {
            windows: HashMap::new(),
            handler,
            width: 0.0,
            height: 0.0,
            top: 0.0,
        }
    }

    pub fn create_window(&mut self, label: &str, url: &str, x: f64, y: f64) {
        // let parent = self.handler.get_window("usopp");

        // js注入测试
        let initialization_script = r#"
        (function() {
            document.addEventListener('DOMContentLoaded', function() {
                const { invoke  } = window.__TAURI__.tauri;
                const { TauriEvent } = window.__TAURI__.event;

                // var invoke = window.__TAURI__.invoke;
                const { appWindow } = window.__TAURI__.window;
                console.log('create window');

                appWindow.listen(TauriEvent.WINDOW_FOCUS, function() {
                    console.log('focus');
                    invoke("window_change", { event: 'focus' });
                });

                appWindow.listen(TauriEvent.WINDOW_BLUR, function() {
                    console.log('blur');
                    invoke("window_change", { event: 'blur' });
                });
            });
            
        })();
        "#;
        if let Some(main_win) = self.get_window("usopp") {
            if let Ok(window) = tauri::WindowBuilder::new(
                &self.handler,
                label,
                tauri::WindowUrl::External(url.parse().unwrap()),
            )
            // .parent_window(parent_win.hwnd().unwrap())
            .owner_window(main_win.hwnd().unwrap())
            .inner_size(self.width, self.height)
            .position(x, y + self.top)
            .decorations(false)
            .transparent(true)
            .initialization_script(initialization_script)
            .build()
            {
                let _ = window.set_focus().unwrap();
                self.windows.insert(label.to_string(), window);
            }
        }
    }

    pub fn set_window(&mut self, label: &str, window: Window) {
        self.windows.insert(label.to_string(), window);
    }

    pub fn get_window(&self, label: &str) -> Option<&Window> {
        self.windows.get(label)
    }

    pub fn update_window_size(&self, width: i32, height: i32) {
        for (_, value) in &self.windows {
            if value.label() != "usopp" {
                let _ = value.set_size(LogicalSize { width, height });
            }
        }
    }

    pub fn update_window_position(&self, position: PhysicalPosition<i32>) {
        let mut _position = position.clone();
        _position.y = _position.y + self.top as i32;
        for (key, value) in &self.windows {
            if key != "usopp" {
                let _ = value.set_position(_position);
            }
        }
    }

    pub fn show_all_window(&self) {
        for (key, _value) in &self.windows {
            let _ = &self.show_window(key);
        }
    }

    pub fn show_window(&self, label: &str) {
        if let Some(window) = self.get_window(label) {
            window.show().expect("Failed to show window");
        }
    }

    pub fn hide_all_window(&self) {
        let mut is_hide = true;
        for (key, _value) in &self.windows {
            // let _ = &self.hide_window(key);
            let win = self.get_window(key);

            if let Some(win) = win {
                // win.hide().expect("Failed to hide window");
                if let Ok(focused) = win.is_focused() {
                    if focused {
                        is_hide = false;
                    }
                }
            }
        }

        if is_hide {
            for (key, _value) in &self.windows {
                // let _ = &self.hide_window(key);
                let win = self.get_window(key);
                if let Some(win) = win {
                    win.hide().expect("Failed to hide window");
                }
            }
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

    pub fn close_child_window(&mut self) {
        let mut labels_to_close = Vec::new();
        for (label, _) in &self.windows {
            if label != "usopp" {
                labels_to_close.push(label.to_string());
            }
        }

        for label in labels_to_close {
            self.close_window(&label);
        }
    }

    pub fn set_window_info(&mut self, info: WindowInfo) {
        self.width = info.width;
        self.height = info.height;
        self.top = info.top;
    }
}
