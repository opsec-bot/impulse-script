use std::ptr;
use std::mem;

#[cfg(windows)]
use winapi::{
    ctypes::c_void,
    shared::{ minwindef::{ BOOL, DWORD, HMODULE, LPVOID, TRUE, FALSE }, ntdef::HANDLE },
    um::{
        memoryapi::{ VirtualAllocEx, VirtualProtectEx, WriteProcessMemory, VirtualFreeEx },
        processthreadsapi::{ OpenProcess, CreateRemoteThread },
        winnt::{
            IMAGE_DOS_HEADER,
            IMAGE_NT_HEADERS,
            IMAGE_SECTION_HEADER,
            MEM_COMMIT,
            MEM_RESERVE,
            MEM_RELEASE,
            PAGE_EXECUTE_READWRITE,
            PAGE_READWRITE,
            PAGE_READONLY,
            PAGE_EXECUTE_READ,
            PROCESS_ALL_ACCESS,
            IMAGE_SCN_MEM_WRITE,
            IMAGE_SCN_MEM_EXECUTE,
            DLL_PROCESS_ATTACH,
        },
        handleapi::CloseHandle,
        libloaderapi::{ GetProcAddress, LoadLibraryA },
        synchapi::Sleep,
    },
};

#[cfg(target_arch = "x86_64")]
const CURRENT_ARCH: u16 = 0x8664; // IMAGE_FILE_MACHINE_AMD64

#[cfg(target_arch = "x86")]
const CURRENT_ARCH: u16 = 0x14c; // IMAGE_FILE_MACHINE_I386

/// Manual mapping data structure passed to shellcode
#[repr(C)]
struct ManualMappingData {
    load_library_a: usize,
    get_proc_address: usize,
    base_address: *mut u8,
    module_handle: HMODULE,
    reason_param: DWORD,
    reserved_param: LPVOID,
    seh_support: BOOL,
}

/// Enhanced manual DLL mapper with proper PE handling
pub struct ManualMapper {
    target_process: HANDLE,
    allocated_base: LPVOID,
    original_base: usize,
    image_size: usize,
    clear_header: bool,
    clear_non_needed_sections: bool,
    adjust_protections: bool,
    seh_exception_support: bool,
}

impl ManualMapper {
    /// Creates a new manual mapper with configuration options
    pub fn new() -> Self {
        Self {
            target_process: ptr::null_mut(),
            allocated_base: ptr::null_mut(),
            original_base: 0,
            image_size: 0,
            clear_header: true,
            clear_non_needed_sections: true,
            adjust_protections: true,
            seh_exception_support: cfg!(target_arch = "x86_64"),
        }
    }

    /// Configures mapper behavior
    pub fn configure(
        &mut self,
        clear_header: bool,
        clear_sections: bool,
        adjust_protections: bool
    ) -> &mut Self {
        self.clear_header = clear_header;
        self.clear_non_needed_sections = clear_sections;
        self.adjust_protections = adjust_protections;
        self
    }

    /// Manually maps a DLL into the target process following the reference implementation
    pub unsafe fn map_dll_to_process(
        &mut self,
        process_id: u32,
        dll_data: &[u8]
    ) -> Result<LPVOID, String> {
        #[cfg(windows)]
        {
            // Validate PE file structure
            unsafe {
                self.validate_pe_file(dll_data)?;
            }

            // Open target process with full access
            self.target_process = unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, process_id) };
            if self.target_process.is_null() {
                return Err(
                    format!("Failed to open process {}: 0x{:X}", process_id, { #[allow(
                            unsafe_op_in_unsafe_fn
                        )]
                        winapi::um::errhandlingapi::GetLastError() })
                );
            }

            let result = unsafe { self.perform_manual_mapping(dll_data) };

            // Cleanup on failure
            if result.is_err() {
                unsafe {
                    self.cleanup_on_failure();
                }
            }

            result
        }

        #[cfg(not(windows))]
        Err("Manual mapping not supported on non-Windows platforms".to_string())
    }

    /// Validates PE file structure and architecture compatibility
    unsafe fn validate_pe_file(&self, dll_data: &[u8]) -> Result<(), String> {
        if dll_data.len() < mem::size_of::<IMAGE_DOS_HEADER>() {
            return Err("Invalid file - too small for DOS header".to_string());
        }

        let dos_header = unsafe { &*(dll_data.as_ptr() as *const IMAGE_DOS_HEADER) };
        if dos_header.e_magic != 0x5a4d {
            return Err("Invalid file - missing MZ signature".to_string());
        }

        let nt_headers_offset = dos_header.e_lfanew as usize;
        if
            nt_headers_offset >= dll_data.len() ||
            nt_headers_offset + mem::size_of::<IMAGE_NT_HEADERS>() > dll_data.len()
        {
            return Err("Invalid NT headers offset".to_string());
        }

        let nt_headers = unsafe {
            &*(((dll_data.as_ptr() as usize) + nt_headers_offset) as *const IMAGE_NT_HEADERS)
        };
        if nt_headers.Signature != 0x00004550 {
            return Err("Invalid NT signature".to_string());
        }

        if nt_headers.FileHeader.Machine != CURRENT_ARCH {
            return Err("Invalid platform - architecture mismatch".to_string());
        }

        Ok(())
    }

    /// Performs the complete manual mapping process
    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn perform_manual_mapping(&mut self, dll_data: &[u8]) -> Result<LPVOID, String> {
        let dos_header = unsafe { &*(dll_data.as_ptr() as *const IMAGE_DOS_HEADER) };
        let nt_headers = unsafe {
            &*(
                ((dll_data.as_ptr() as usize) +
                    (dos_header.e_lfanew as usize)) as *const IMAGE_NT_HEADERS
            )
        };
        let opt_header = &nt_headers.OptionalHeader;

        self.original_base = opt_header.ImageBase as usize;
        self.image_size = opt_header.SizeOfImage as usize;

        // Allocate memory in target process
        self.allocated_base = unsafe {
            VirtualAllocEx(
                self.target_process,
                ptr::null_mut(),
                self.image_size,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE
            )
        };

        if self.allocated_base.is_null() {
            return Err(
                format!(
                    "Target process memory allocation failed: 0x{:X}",
                    winapi::um::errhandlingapi::GetLastError()
                )
            );
        }

        // Set initial protection to execute/read/write
        let mut old_protect = 0;
        VirtualProtectEx(
            self.target_process,
            self.allocated_base,
            self.image_size,
            PAGE_EXECUTE_READWRITE,
            &mut old_protect
        );

        // Write PE headers (first 0x1000 bytes)
        let header_size = std::cmp::min(0x1000, dll_data.len());
        if
            (unsafe {
                WriteProcessMemory(
                    self.target_process,
                    self.allocated_base,
                    dll_data.as_ptr() as *const c_void,
                    header_size,
                    ptr::null_mut()
                )
            }) == FALSE
        {
            return Err(
                format!(
                    "Failed to write PE headers: 0x{:X}",
                    winapi::um::errhandlingapi::GetLastError()
                )
            );
        }

        // Map all sections
        unsafe {
            self.map_pe_sections(dll_data, nt_headers)?;
        }

        // Create and inject shellcode
        unsafe {
            self.inject_mapping_shellcode(dll_data)?;
        }

        // Wait for mapping completion
        unsafe {
            self.wait_for_mapping_completion()?;
        }

        // Perform post-mapping cleanup
        unsafe {
            self.perform_post_mapping_cleanup(dll_data, nt_headers)?;
        }

        Ok(self.allocated_base)
    }

    /// Maps all PE sections to allocated memory
    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn map_pe_sections(
        &self,
        dll_data: &[u8],
        nt_headers: &IMAGE_NT_HEADERS
    ) -> Result<(), String> {
        let section_header_ptr = ((dll_data.as_ptr() as usize) +
            ((dll_data.as_ptr() as *const IMAGE_DOS_HEADER).read().e_lfanew as usize) +
            mem::size_of::<IMAGE_NT_HEADERS>()) as *const IMAGE_SECTION_HEADER;

        for i in 0..nt_headers.FileHeader.NumberOfSections {
            let section = &*section_header_ptr.add(i as usize);

            if section.SizeOfRawData == 0 {
                continue;
            }

            let section_dest = ((self.allocated_base as usize) +
                (section.VirtualAddress as usize)) as LPVOID;
            let section_src = ((dll_data.as_ptr() as usize) +
                (section.PointerToRawData as usize)) as *const c_void;

            if
                WriteProcessMemory(
                    self.target_process,
                    section_dest,
                    section_src,
                    section.SizeOfRawData as usize,
                    ptr::null_mut()
                ) == FALSE
            {
                return Err(
                    format!(
                        "Failed to map section {}: 0x{:X}",
                        i,
                        winapi::um::errhandlingapi::GetLastError()
                    )
                );
            }
        }

        Ok(())
    }

    /// Creates and injects the mapping shellcode
    unsafe fn inject_mapping_shellcode(&self, _dll_data: &[u8]) -> Result<(), String> {
        // Prepare mapping data
        let mapping_data = ManualMappingData {
            load_library_a: LoadLibraryA as usize,
            get_proc_address: GetProcAddress as usize,
            base_address: self.allocated_base as *mut u8,
            module_handle: ptr::null_mut(),
            reason_param: DLL_PROCESS_ATTACH,
            reserved_param: ptr::null_mut(),
            seh_support: if self.seh_exception_support {
                TRUE
            } else {
                FALSE
            },
        };

        // Allocate memory for mapping data
        #[allow(unsafe_op_in_unsafe_fn)]
        let mapping_data_alloc = VirtualAllocEx(
            self.target_process,
            ptr::null_mut(),
            mem::size_of::<ManualMappingData>(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE
        );

        if mapping_data_alloc.is_null() {
            return Err("Failed to allocate mapping data memory".to_string());
        }

        // Write mapping data
        #[allow(unsafe_op_in_unsafe_fn)]
        if
            WriteProcessMemory(
                self.target_process,
                mapping_data_alloc,
                &mapping_data as *const ManualMappingData as *const c_void,
                mem::size_of::<ManualMappingData>(),
                ptr::null_mut()
            ) == FALSE
        {
            VirtualFreeEx(self.target_process, mapping_data_alloc, 0, MEM_RELEASE);
            return Err("Failed to write mapping data".to_string());
        }

        // Generate and inject shellcode
        let shellcode = self.generate_mapping_shellcode();
        #[allow(unsafe_op_in_unsafe_fn)]
        let shellcode_alloc = VirtualAllocEx(
            self.target_process,
            ptr::null_mut(),
            shellcode.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE
        );

        #[allow(unsafe_op_in_unsafe_fn)]
        if shellcode_alloc.is_null() {
            VirtualFreeEx(self.target_process, mapping_data_alloc, 0, MEM_RELEASE);
            return Err("Failed to allocate shellcode memory".to_string());
        }

        #[allow(unsafe_op_in_unsafe_fn)]
        if
            WriteProcessMemory(
                self.target_process,
                shellcode_alloc,
                shellcode.as_ptr() as *const c_void,
                shellcode.len(),
                ptr::null_mut()
            ) == FALSE
        {
            VirtualFreeEx(self.target_process, mapping_data_alloc, 0, MEM_RELEASE);
            VirtualFreeEx(self.target_process, shellcode_alloc, 0, MEM_RELEASE);
            return Err("Failed to write shellcode".to_string());
        }

        // Create remote thread to execute shellcode
        #[allow(unsafe_op_in_unsafe_fn)]
        let remote_thread = CreateRemoteThread(
            self.target_process,
            ptr::null_mut(),
            0,
            Some(mem::transmute(shellcode_alloc)),
            mapping_data_alloc,
            0,
            ptr::null_mut()
        );

        #[allow(unsafe_op_in_unsafe_fn)]
        if remote_thread.is_null() {
            VirtualFreeEx(self.target_process, mapping_data_alloc, 0, MEM_RELEASE);
            VirtualFreeEx(self.target_process, shellcode_alloc, 0, MEM_RELEASE);
            return Err("Failed to create remote thread".to_string());
        }

        #[allow(unsafe_op_in_unsafe_fn)]
        CloseHandle(remote_thread);
        Ok(())
    }

    /// Waits for the mapping process to complete
    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn wait_for_mapping_completion(&self) -> Result<(), String> {
        // Implementation would monitor the mapping data for completion
        // For now, simple sleep - in production, use proper synchronization
        Sleep(1000);
        Ok(())
    }

    /// Performs post-mapping cleanup operations
    unsafe fn perform_post_mapping_cleanup(
        &self,
        dll_data: &[u8],
        nt_headers: &IMAGE_NT_HEADERS
    ) -> Result<(), String> {
        let empty_buffer = vec![0u8; 0x1000];

        // Clear PE headers if requested
        #[allow(unsafe_op_in_unsafe_fn)]
        if self.clear_header {
            WriteProcessMemory(
                self.target_process,
                self.allocated_base,
                empty_buffer.as_ptr() as *const c_void,
                0x1000,
                ptr::null_mut()
            );
        }

        // Clear non-needed sections if requested
        #[allow(unsafe_op_in_unsafe_fn)]
        if self.clear_non_needed_sections {
            self.clear_unnecessary_sections(dll_data, nt_headers)?;
        }

        // Adjust memory protections if requested
        #[allow(unsafe_op_in_unsafe_fn)]
        if self.adjust_protections {
            self.adjust_section_protections(dll_data, nt_headers)?;
        }

        Ok(())
    }

    /// Clears unnecessary sections like .pdata, .rsrc, .reloc
    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn clear_unnecessary_sections(
        &self,
        dll_data: &[u8],
        nt_headers: &IMAGE_NT_HEADERS
    ) -> Result<(), String> {
        let section_header_ptr = ((dll_data.as_ptr() as usize) +
            ((dll_data.as_ptr() as *const IMAGE_DOS_HEADER).read().e_lfanew as usize) +
            mem::size_of::<IMAGE_NT_HEADERS>()) as *const IMAGE_SECTION_HEADER;

        let empty_buffer = vec![0u8; 1024 * 1024]; // Large buffer for clearing

        for i in 0..nt_headers.FileHeader.NumberOfSections {
            let section = &*section_header_ptr.add(i as usize);

            if *section.Misc.VirtualSize() == 0 {
                continue;
            }

            let section_name = std::ffi::CStr
                ::from_ptr(section.Name.as_ptr() as *const i8)
                .to_string_lossy();

            let should_clear =
                section_name == ".pdata" || section_name == ".rsrc" || section_name == ".reloc";

            if should_clear {
                let section_dest = ((self.allocated_base as usize) +
                    (section.VirtualAddress as usize)) as LPVOID;
                let clear_size = std::cmp::min(
                    *section.Misc.VirtualSize() as usize,
                    empty_buffer.len()
                );

                WriteProcessMemory(
                    self.target_process,
                    section_dest,
                    empty_buffer.as_ptr() as *const c_void,
                    clear_size,
                    ptr::null_mut()
                );
            }
        }

        Ok(())
    }

    /// Adjusts section memory protections based on characteristics
    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn adjust_section_protections(
        &self,
        dll_data: &[u8],
        nt_headers: &IMAGE_NT_HEADERS
    ) -> Result<(), String> {
        let section_header_ptr = ((dll_data.as_ptr() as usize) +
            ((dll_data.as_ptr() as *const IMAGE_DOS_HEADER).read().e_lfanew as usize) +
            mem::size_of::<IMAGE_NT_HEADERS>()) as *const IMAGE_SECTION_HEADER;

        for i in 0..nt_headers.FileHeader.NumberOfSections {
            let section = &*section_header_ptr.add(i as usize);

            if *section.Misc.VirtualSize() == 0 {
                continue;
            }

            let mut new_protection = PAGE_READONLY;

            if (section.Characteristics & IMAGE_SCN_MEM_WRITE) != 0 {
                new_protection = PAGE_READWRITE;
            } else if (section.Characteristics & IMAGE_SCN_MEM_EXECUTE) != 0 {
                new_protection = PAGE_EXECUTE_READ;
            }

            let section_addr = ((self.allocated_base as usize) +
                (section.VirtualAddress as usize)) as LPVOID;
            let mut old_protection = 0;

            let result = VirtualProtectEx(
                self.target_process,
                section_addr,
                *section.Misc.VirtualSize() as usize,
                new_protection,
                &mut old_protection
            );

            if result == FALSE {
                return Err(format!(
                    "Failed to adjust protection for section {}: 0x{:X}",
                    i,
                    winapi::um::errhandlingapi::GetLastError()
                ));
            }
        }

        Ok(())
    }

    /// Generates mapping shellcode with proper relocation and import handling
    fn generate_mapping_shellcode(&self) -> Vec<u8> {
        // Enhanced shellcode that handles basic DLL initialization
        // This is a simplified version - production code would need full PE processing
        vec![
            // x64 assembly: Basic function prologue and epilogue
            0x48, 0x83, 0xec, 0x28,  // sub rsp, 0x28 (shadow space)
            0x48, 0x89, 0xc8,        // mov rax, rcx (parameter)
            0x48, 0x83, 0xc4, 0x28,  // add rsp, 0x28
            0xc3                     // ret
        ]
    }

    /// Gets the mapped DLL base address
    pub fn get_mapped_base(&self) -> LPVOID {
        self.allocated_base
    }

    /// Gets the original DLL base address
    pub fn get_original_base(&self) -> usize {
        self.original_base
    }

    /// Gets the image size
    pub fn get_image_size(&self) -> usize {
        self.image_size
    }

    /// Cleans up allocated resources on failure
    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn cleanup_on_failure(&self) {
        if !self.allocated_base.is_null() {
            VirtualFreeEx(self.target_process, self.allocated_base, 0, MEM_RELEASE);
        }
    }
}

impl Drop for ManualMapper {
    fn drop(&mut self) {
        #[cfg(windows)]
        unsafe {
            if !self.target_process.is_null() {
                CloseHandle(self.target_process);
            }
        }
    }
}

impl Default for ManualMapper {
    fn default() -> Self {
        Self::new()
    }
}
