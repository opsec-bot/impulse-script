use std::ptr;

#[cfg(windows)]
use winapi::{
    shared::{ windef::HWND, minwindef::{ DWORD, BOOL, TRUE } },
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

const WDA_EXCLUDEFROMCAPTURE: DWORD = 0x00000011;
const WDA_NONE: DWORD = 0x00000000;

pub struct ProcessGhost {
    pub window_handle: Option<HWND>,
    is_hidden_from_alt_tab: bool,
    is_hidden_from_capture: bool,
    current_process_id: u32,
}

impl ProcessGhost {
    pub fn new() -> Self {
        println!("🚀 Initializing ProcessGhost with advanced DLL injection capabilities");
        Self {
            window_handle: None,
            is_hidden_from_alt_tab: false,
            is_hidden_from_capture: false,
            current_process_id: Self::get_current_process_id_internal(),
        }
    }

    fn get_current_process_id_internal() -> u32 {
        #[cfg(windows)]
        unsafe {
            GetCurrentProcessId()
        }

        #[cfg(not(windows))]
        0
    }

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

    pub fn hide_from_alt_tab(&mut self) -> Result<(), String> {
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
                        let current_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                        let new_style =
                            (current_style & !(WS_EX_APPWINDOW as isize)) |
                            (WS_EX_TOOLWINDOW as isize);

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
                self.is_hidden_from_alt_tab = true;
                Ok(())
            } else {
                Err("No windows found to hide".to_string())
            }
        }

        #[cfg(not(windows))]
        Err("Windows API not available on this platform".to_string())
    }

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
                Ok(())
            } else {
                Err("No windows found to restore".to_string())
            }
        }

        #[cfg(not(windows))]
        Err("Windows API not available on this platform".to_string())
    }

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
                Ok(())
            } else {
                Err("No windows found to hide from screen capture".to_string())
            }
        }

        #[cfg(not(windows))]
        Err("Windows API not available on this platform".to_string())
    }

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
                Ok(())
            } else {
                Err("No windows found to restore to screen capture".to_string())
            }
        }

        #[cfg(not(windows))]
        Err("Windows API not available on this platform".to_string())
    }
}
impl Default for ProcessGhost {
    fn default() -> Self {
        Self::new()
    }
}
