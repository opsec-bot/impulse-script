use std::ffi::CString;
use std::ptr;

#[cfg(windows)]
use winapi::{
    ctypes::c_void,
    shared::minwindef::{ TRUE, FALSE },
    um::{
        processthreadsapi::{ OpenProcess, CreateRemoteThread },
        memoryapi::{ VirtualAllocEx, WriteProcessMemory },
        libloaderapi::{ GetModuleHandleA, GetProcAddress },
        handleapi::CloseHandle,
        winnt::{
            PROCESS_CREATE_THREAD,
            PROCESS_QUERY_INFORMATION,
            PROCESS_VM_READ,
            PROCESS_VM_WRITE,
            PROCESS_VM_OPERATION,
            MEM_COMMIT,
            PAGE_READWRITE,
        },
        tlhelp32::{
            CreateToolhelp32Snapshot,
            Process32First,
            Process32Next,
            PROCESSENTRY32,
            TH32CS_SNAPPROCESS,
        },
    },
};

use super::manual_mapper::ManualMapper;

#[allow(dead_code)]
pub struct DllInjector {
    hide_screenshare_dll: String,
    unhide_screenshare_dll: String,
    hide_taskbar_dll: String,
    unhide_taskbar_dll: String,
    manual_mapper: ManualMapper,
}

#[allow(dead_code)]
impl DllInjector {
    pub fn new() -> Self {
        Self {
            hide_screenshare_dll: "./lib/hide_screenshare.dll".to_string(),
            unhide_screenshare_dll: "./lib/unhide_screenshare.dll".to_string(),
            hide_taskbar_dll: "./lib/hide_taskbar.dll".to_string(),
            unhide_taskbar_dll: "./lib/unhide_taskbar.dll".to_string(),
            manual_mapper: ManualMapper::new(),
        }
    }

    pub fn inject_dll(&self, pid: u32, dll_path: &str) -> Result<(), String> {
        #[cfg(windows)]
        unsafe {
            let process_handle = OpenProcess(
                PROCESS_CREATE_THREAD |
                    PROCESS_QUERY_INFORMATION |
                    PROCESS_VM_READ |
                    PROCESS_VM_WRITE |
                    PROCESS_VM_OPERATION,
                FALSE,
                pid
            );

            if process_handle.is_null() {
                return Err(format!("Failed to open process {}", pid));
            }

            let kernel32_handle = GetModuleHandleA(b"kernel32.dll\0".as_ptr() as *const i8);
            if kernel32_handle.is_null() {
                CloseHandle(process_handle);
                return Err("Failed to get kernel32.dll handle".to_string());
            }

            let load_library_addr = GetProcAddress(
                kernel32_handle,
                b"LoadLibraryA\0".as_ptr() as *const i8
            );
            if load_library_addr.is_null() {
                CloseHandle(process_handle);
                return Err("Failed to get LoadLibraryA address".to_string());
            }

            let dll_path_cstring = CString::new(dll_path).map_err(|_| "Invalid DLL path")?;
            let dll_path_len = dll_path_cstring.as_bytes_with_nul().len();

            let allocated_memory = VirtualAllocEx(
                process_handle,
                ptr::null_mut(),
                dll_path_len,
                MEM_COMMIT,
                PAGE_READWRITE
            );

            if allocated_memory.is_null() {
                CloseHandle(process_handle);
                return Err("Failed to allocate memory in target process".to_string());
            }

            let mut bytes_written = 0;
            let write_result = WriteProcessMemory(
                process_handle,
                allocated_memory,
                dll_path_cstring.as_ptr() as *const c_void,
                dll_path_len,
                &mut bytes_written
            );

            if write_result == FALSE {
                CloseHandle(process_handle);
                return Err("Failed to write DLL path to target process".to_string());
            }

            let remote_thread = CreateRemoteThread(
                process_handle,
                ptr::null_mut(),
                0,
                Some(std::mem::transmute(load_library_addr)),
                allocated_memory,
                0,
                ptr::null_mut()
            );

            if remote_thread.is_null() {
                CloseHandle(process_handle);
                return Err("Failed to create remote thread".to_string());
            }

            CloseHandle(remote_thread);
            CloseHandle(process_handle);

            Ok(())
        }

        #[cfg(not(windows))]
        Err("DLL injection not supported on non-Windows platforms".to_string())
    }

    /// Injects a DLL using enhanced manual mapping technique
    pub fn inject_dll_manual_map(&mut self, pid: u32, dll_path: &str) -> Result<(), String> {
        // Validate DLL file exists
        if !std::path::Path::new(dll_path).exists() {
            return Err(format!("DLL file not found: {}", dll_path));
        }

        // Read DLL file into memory
        let dll_data = std::fs::read(dll_path)
            .map_err(|e| format!("Failed to read DLL file '{}': {}", dll_path, e))?;

        // Validate minimum file size
        if dll_data.len() < 0x1000 {
            return Err(format!("Invalid DLL file size: {} bytes", dll_data.len()));
        }

        // Configure manual mapper with enhanced stealth settings
        self.manual_mapper.configure(true, true, true);

        unsafe {
            match self.manual_mapper.map_dll_to_process(pid, &dll_data) {
                Ok(base_addr) => {
                    println!(
                        "DLL '{}' successfully mapped to process {} at address: {:p}",
                        dll_path, pid, base_addr
                    );
                    Ok(())
                }
                Err(e) => Err(format!("Manual mapping of '{}' failed: {}", dll_path, e))
            }
        }
    }

    /// Build dynamic path from components
    fn build_dynamic_path(components: &[&str]) -> String {
        components.join("")
    }

    /// Enhanced hiding method with comprehensive stealth features
    pub fn hide_from_screenshare_stealth(&mut self, pid: u32) -> Result<(), String> {
        // Use dynamic path construction to avoid static string detection
        let obfuscated_path = Self::build_dynamic_path(&[".", "/", "lib", "/", "hide_screenshare.dll"]);
        
        println!("Attempting stealth injection to process {} using manual mapping", pid);
        self.inject_dll_manual_map(pid, &obfuscated_path)
    }

    /// Advanced injection method with fallback strategy
    pub fn inject_dll_with_fallback(&mut self, pid: u32, dll_path: &str, prefer_manual_map: bool) -> Result<(), String> {
        if prefer_manual_map {
            // Try manual mapping first
            match self.inject_dll_manual_map(pid, dll_path) {
                Ok(()) => {
                    println!("Manual mapping injection successful for PID {}", pid);
                    Ok(())
                }
                Err(manual_err) => {
                    println!("Manual mapping failed: {}, falling back to standard injection", manual_err);
                    // Fallback to standard injection
                    self.inject_dll(pid, dll_path)
                        .map_err(|std_err| format!("Both injection methods failed. Manual: {}, Standard: {}", manual_err, std_err))
                }
            }
        } else {
            // Try standard injection first
            match self.inject_dll(pid, dll_path) {
                Ok(()) => {
                    println!("Standard injection successful for PID {}", pid);
                    Ok(())
                }
                Err(std_err) => {
                    println!("Standard injection failed: {}, falling back to manual mapping", std_err);
                    // Fallback to manual mapping
                    self.inject_dll_manual_map(pid, dll_path)
                        .map_err(|manual_err| format!("Both injection methods failed. Standard: {}, Manual: {}", std_err, manual_err))
                }
            }
        }
    }

    /// Hide from screenshare with choice of injection method and fallback
    pub fn hide_from_screenshare_advanced(
        &mut self,
        pid: u32,
        use_manual_map: bool
    ) -> Result<(), String> {
        if use_manual_map {
            self.hide_from_screenshare_stealth(pid)
        } else {
            self.inject_dll_with_fallback(pid, &self.hide_screenshare_dll.clone(), false)
        }
    }

    /// Hide from taskbar with choice of injection method and fallback
    pub fn hide_from_taskbar_advanced(
        &mut self,
        pid: u32,
        use_manual_map: bool
    ) -> Result<(), String> {
        let dll_path = self.hide_taskbar_dll.clone();
        if use_manual_map {
            self.inject_dll_manual_map(pid, &dll_path)
        } else {
            self.inject_dll_with_fallback(pid, &dll_path, false)
        }
    }

    /// Gets information about the last manual mapping operation
    pub fn get_mapping_info(&self) -> Option<(usize, usize, *const c_void)> {
        if !self.manual_mapper.get_mapped_base().is_null() {
            Some((
                self.manual_mapper.get_original_base(),
                self.manual_mapper.get_image_size(),
                self.manual_mapper.get_mapped_base()
            ))
        } else {
            None
        }
    }

    pub fn find_process_by_name(&self, process_name: &str) -> Vec<u32> {
        #[cfg(windows)]
        unsafe {
            let mut pids = Vec::new();
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

            if snapshot == ptr::null_mut() {
                return pids;
            }

            let mut process_entry: PROCESSENTRY32 = std::mem::zeroed();
            process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

            if Process32First(snapshot, &mut process_entry) == TRUE {
                loop {
                    let current_process_name = std::ffi::CStr
                        ::from_ptr(process_entry.szExeFile.as_ptr())
                        .to_string_lossy()
                        .to_lowercase();

                    let search_name = process_name.to_lowercase();
                    if
                        current_process_name == search_name ||
                        current_process_name == format!("{}.exe", search_name)
                    {
                        pids.push(process_entry.th32ProcessID);
                    }

                    if Process32Next(snapshot, &mut process_entry) == FALSE {
                        break;
                    }
                }
            }

            CloseHandle(snapshot);
            pids
        }

        #[cfg(not(windows))]
        Vec::new()
    }

    pub fn get_current_process_id(&self) -> u32 {
        #[cfg(windows)]
        unsafe {
            winapi::um::processthreadsapi::GetCurrentProcessId()
        }

        #[cfg(not(windows))]
        0
    }

    pub fn validate_dlls(&self) -> Result<(), String> {
        let dlls = [
            &self.hide_screenshare_dll,
            &self.unhide_screenshare_dll,
            &self.hide_taskbar_dll,
            &self.unhide_taskbar_dll,
        ];

        for dll_path in &dlls {
            if !std::path::Path::new(dll_path).exists() {
                return Err(format!("DLL not found: {}", dll_path));
            }
        }

        Ok(())
    }
}

impl Default for DllInjector {
    fn default() -> Self {
        Self::new()
    }
}
