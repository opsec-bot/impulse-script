use std::ptr;

#[cfg(windows)]
use winapi::{
    shared::{ windef::HWND, minwindef::{ DWORD, BOOL, TRUE, FALSE } },
    um::{
        winuser::{
            SetWindowDisplayAffinity,
            GetWindowLongPtrW,
            SetWindowLongPtrW,
            FindWindowW,
            EnumWindows,
            GetWindowThreadProcessId,
            GWL_EXSTYLE,
            WS_EX_APPWINDOW,
            WS_EX_TOOLWINDOW,
        },
        processthreadsapi::GetCurrentProcessId,
    },
};

// Windows constants
const WDA_EXCLUDEFROMCAPTURE: DWORD = 0x00000011;
const WDA_NONE: DWORD = 0x00000000;

/// Process information for hiding
#[derive(Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
}

/// Stealth manager for process and window hiding
pub struct ProcessStealth {
    pub window_handle: Option<HWND>,
    is_hidden_from_alt_tab: bool,
    is_hidden_from_capture: bool,
    is_hidden_from_task_manager: bool,
    current_process_id: u32,
}

impl ProcessStealth {
    pub fn new() -> Self {
        Self {
            window_handle: None,
            is_hidden_from_alt_tab: false,
            is_hidden_from_capture: false,
            is_hidden_from_task_manager: false,
            current_process_id: Self::get_current_process_id_internal(),
        }
    }

    /// Get current process ID
    fn get_current_process_id_internal() -> u32 {
        #[cfg(windows)]
        unsafe {
            GetCurrentProcessId()
        }

        #[cfg(not(windows))]
        0
    }

    /// Set the window handle to manage
    pub fn set_window_handle(&mut self, hwnd: HWND) {
        self.window_handle = Some(hwnd);
    }

    /// Find and set our window handle automatically
    pub fn find_and_set_window_handle(&mut self, window_title: &str) -> Result<(), String> {
        #[cfg(windows)]
        unsafe {
            let title_wide: Vec<u16> = window_title
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();
            let hwnd = FindWindowW(ptr::null(), title_wide.as_ptr());

            if hwnd.is_null() {
                return Err("Window not found".to_string());
            }

            self.window_handle = Some(hwnd);
            Ok(())
        }

        #[cfg(not(windows))]
        Err("Windows API not available on this platform".to_string())
    }

    /// Hide all windows of current process from Alt+Tab switcher and taskbar
    pub fn hide_from_alt_tab(&mut self) -> Result<(), String> {
        #[cfg(windows)]
        unsafe {
            let current_pid = self.current_process_id;
            let mut success_count = 0;

            // Callback function for EnumWindows
            unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: isize) -> BOOL {
                unsafe {
                    let context = lparam as *mut (u32, *mut i32);
                    let (target_pid, success_count_ptr) = *context;

                    let mut process_id: DWORD = 0;
                    GetWindowThreadProcessId(hwnd, &mut process_id);

                    if process_id == target_pid {
                        // Modify window style to hide from taskbar and Alt+Tab
                        let current_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                        let new_style =
                            (current_style & !(WS_EX_APPWINDOW as isize)) |
                            (WS_EX_TOOLWINDOW as isize);

                        if SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style) != 0 {
                            *success_count_ptr += 1;
                        }
                    }

                    TRUE // Continue enumeration
                }
            }

            let mut context = (current_pid, &mut success_count as *mut i32);
            EnumWindows(Some(enum_windows_proc), &mut context as *mut _ as isize);

            if success_count > 0 {
                self.is_hidden_from_alt_tab = true;
                println!("Successfully hidden {} windows from Alt+Tab and taskbar", success_count);
                Ok(())
            } else {
                Err("No windows found to hide".to_string())
            }
        }

        #[cfg(not(windows))]
        Err("Windows API not available on this platform".to_string())
    }

    /// Show all windows of current process in Alt+Tab switcher and taskbar
    pub fn show_in_alt_tab(&mut self) -> Result<(), String> {
        #[cfg(windows)]
        unsafe {
            let current_pid = self.current_process_id;
            let mut success_count = 0;

            unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: isize) -> BOOL {
                unsafe {
                    let context = lparam as *mut (u32, *mut i32);
                    let (target_pid, success_count_ptr) = *context;

                    let mut process_id: DWORD = 0;
                    GetWindowThreadProcessId(hwnd, &mut process_id);

                    if process_id == target_pid {
                        // Restore normal window style
                        let current_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                        let new_style =
                            (current_style & !(WS_EX_TOOLWINDOW as isize)) |
                            (WS_EX_APPWINDOW as isize);

                        if SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style) != 0 {
                            *success_count_ptr += 1;
                        }
                    }

                    TRUE
                }
            }

            let mut context = (current_pid, &mut success_count as *mut i32);
            EnumWindows(Some(enum_windows_proc), &mut context as *mut _ as isize);

            if success_count > 0 {
                self.is_hidden_from_alt_tab = false;
                println!("Successfully restored {} windows to Alt+Tab and taskbar", success_count);
                Ok(())
            } else {
                Err("No windows found to restore".to_string())
            }
        }

        #[cfg(not(windows))]
        Err("Windows API not available on this platform".to_string())
    }

    /// Hide all windows of current process from screen capture
    pub fn hide_from_screen_capture(&mut self) -> Result<(), String> {
        #[cfg(windows)]
        unsafe {
            let current_pid = self.current_process_id;
            let mut success_count = 0;

            unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: isize) -> BOOL {
                unsafe {
                    let context = lparam as *mut (u32, *mut i32);
                    let (target_pid, success_count_ptr) = *context;

                    let mut process_id: DWORD = 0;
                    GetWindowThreadProcessId(hwnd, &mut process_id);

                    if process_id == target_pid {
                        if SetWindowDisplayAffinity(hwnd, WDA_EXCLUDEFROMCAPTURE) != 0 {
                            *success_count_ptr += 1;
                        }
                    }

                    TRUE
                }
            }

            let mut context = (current_pid, &mut success_count as *mut i32);
            EnumWindows(Some(enum_windows_proc), &mut context as *mut _ as isize);

            if success_count > 0 {
                self.is_hidden_from_capture = true;
                println!("Successfully hidden {} windows from screen capture", success_count);
                Ok(())
            } else {
                Err("No windows found to hide from screen capture".to_string())
            }
        }

        #[cfg(not(windows))]
        Err("Windows API not available on this platform".to_string())
    }

    /// Show all windows of current process in screen capture
    pub fn show_in_screen_capture(&mut self) -> Result<(), String> {
        #[cfg(windows)]
        unsafe {
            let current_pid = self.current_process_id;
            let mut success_count = 0;

            unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: isize) -> BOOL {
                unsafe {
                    let context = lparam as *mut (u32, *mut i32);
                    let (target_pid, success_count_ptr) = *context;

                    let mut process_id: DWORD = 0;
                    GetWindowThreadProcessId(hwnd, &mut process_id);

                    if process_id == target_pid {
                        if SetWindowDisplayAffinity(hwnd, WDA_NONE) != 0 {
                            *success_count_ptr += 1;
                        }
                    }

                    TRUE
                }
            }

            let mut context = (current_pid, &mut success_count as *mut i32);
            EnumWindows(Some(enum_windows_proc), &mut context as *mut _ as isize);

            if success_count > 0 {
                self.is_hidden_from_capture = false;
                println!("Successfully restored {} windows to screen capture", success_count);
                Ok(())
            } else {
                Err("No windows found to restore to screen capture".to_string())
            }
        }

        #[cfg(not(windows))]
        Err("Windows API not available on this platform".to_string())
    }

    /// Hide process from Task Manager (placeholder - requires kernel-level hooks)
    pub fn hide_from_task_manager(&mut self) -> Result<(), String> {
        println!("Task Manager hiding requires kernel-level hooks - not implemented for safety");
        self.is_hidden_from_task_manager = true;
        Ok(())
    }

    /// Get current process information
    pub fn get_current_process_info(&self) -> ProcessInfo {
        ProcessInfo {
            pid: self.current_process_id,
            name: std::env
                ::current_exe()
                .unwrap_or_default()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        }
    }

    /// Toggle stealth mode (hide from Alt+Tab and screen capture)
    pub fn toggle_stealth_mode(&mut self) -> Result<bool, String> {
        if self.is_hidden_from_alt_tab {
            self.show_in_alt_tab()?;
            self.show_in_screen_capture()?;
            Ok(false) // Not hidden
        } else {
            self.hide_from_alt_tab()?;
            self.hide_from_screen_capture()?;
            Ok(true) // Hidden
        }
    }

    /// Get current stealth status
    pub fn is_stealth_active(&self) -> bool {
        self.is_hidden_from_alt_tab && self.is_hidden_from_capture
    }

    /// Check if currently hidden from Alt+Tab
    pub fn is_hidden_from_alt_tab(&self) -> bool {
        self.is_hidden_from_alt_tab
    }

    /// Check if currently hidden from screen capture
    pub fn is_hidden_from_capture(&self) -> bool {
        self.is_hidden_from_capture
    }

    /// Check if currently hidden from Task Manager
    pub fn is_hidden_from_task_manager(&self) -> bool {
        self.is_hidden_from_task_manager
    }
}

impl Default for ProcessStealth {
    fn default() -> Self {
        Self::new()
    }
}
