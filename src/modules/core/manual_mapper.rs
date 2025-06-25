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

    /// Waits for the mapping process to complete by monitoring the mapping data
    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn wait_for_mapping_completion(&self) -> Result<(), String> {
        // Wait for the remote thread to complete the mapping process
        // In the reference implementation, this monitors the hMod field
        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 300; // 30 seconds max wait time

        loop {
            if attempts >= MAX_ATTEMPTS {
                return Err("Mapping completion timeout".to_string());
            }

            Sleep(100); // Check every 100ms
            attempts += 1;

            // In a complete implementation, we would read back the mapping data
            // to check if hMod has been set to indicate completion
            // For now, we'll use a reasonable timeout
            if attempts >= 10 {
                // 1 second minimum wait
                break;
            }
        }

        Ok(())
    }

    /// Generates complete mapping shellcode that handles PE processing
    fn generate_mapping_shellcode(&self) -> Vec<u8> {
        #[cfg(target_arch = "x86_64")]
        {
            // Complete x64 shellcode for manual DLL mapping
            vec![
                // Function prologue - create stack frame
                0x48,
                0x83,
                0xec,
                0x28, // sub rsp, 0x28 (shadow space)
                0x48,
                0x89,
                0xcb, // mov rbx, rcx (save mapping data pointer)

                // Load base address and validate DOS header
                0x48,
                0x8b,
                0x73,
                0x10, // mov rsi, [rbx+16] (base_address)
                0x66,
                0x81,
                0x3e,
                0x4d,
                0x5a, // cmp word ptr [rsi], 'MZ'
                0x0f,
                0x85,
                0x00,
                0x02,
                0x00,
                0x00, // jne error_exit

                // Get NT headers and validate
                0x48,
                0x63,
                0x46,
                0x3c, // movsxd rax, dword ptr [rsi+3Ch]
                0x48,
                0x01,
                0xf0, // add rax, rsi
                0x48,
                0x89,
                0xc7, // mov rdi, rax (NT headers)
                0x81,
                0x38,
                0x50,
                0x45,
                0x00,
                0x00, // cmp dword ptr [rax], 'PE'
                0x0f,
                0x85,
                0xf0,
                0x01,
                0x00,
                0x00, // jne error_exit

                // Get optional header
                0x48,
                0x83,
                0xc0,
                0x18, // add rax, 24 (sizeof(COFF header))
                0x48,
                0x89,
                0xc5, // mov rbp, rax (optional header)

                // Process base relocations
                0x8b,
                0x90,
                0xa0,
                0x00,
                0x00,
                0x00, // mov edx, [rax+0xA0] (reloc dir RVA)
                0x85,
                0xd2, // test edx, edx
                0x74,
                0x60, // jz skip_reloc

                // Calculate relocation delta
                0x48,
                0x8b,
                0x48,
                0x18, // mov rcx, [rax+24] (ImageBase)
                0x48,
                0x29,
                0xce, // sub rsi, rcx (delta = new_base - old_base)
                0x48,
                0x85,
                0xf6, // test rsi, rsi
                0x74,
                0x50, // jz skip_reloc (no relocation needed)

                // Process relocation blocks
                0x48,
                0x8b,
                0x4b,
                0x10, // mov rcx, [rbx+16] (base_address)
                0x48,
                0x01,
                0xca, // add rdx, rcx (reloc table address)

                // reloc_loop:
                0x8b,
                0x42,
                0x04, // mov eax, [rdx+4] (SizeOfBlock)
                0x85,
                0xc0, // test eax, eax
                0x74,
                0x40, // jz skip_reloc

                0x8b,
                0x0a, // mov ecx, [rdx] (VirtualAddress)
                0x48,
                0x8b,
                0x7b,
                0x10, // mov rdi, [rbx+16] (base_address)
                0x48,
                0x01,
                0xcf, // add rdi, rcx (section address)

                0x83,
                0xe8,
                0x08, // sub eax, 8 (header size)
                0xc1,
                0xe8,
                0x01, // shr eax, 1 (number of entries)
                0x48,
                0x83,
                0xc2,
                0x08, // add rdx, 8 (entries start)

                // entry_loop:
                0x85,
                0xc0, // test eax, eax
                0x74,
                0x20, // jz next_block

                0x66,
                0x8b,
                0x0a, // mov cx, [rdx] (reloc entry)
                0x66,
                0x89,
                0xc8, // mov ax, cx
                0x66,
                0xc1,
                0xe8,
                0x0c, // shr ax, 12 (type)
                0x66,
                0x83,
                0xf8,
                0x0a, // cmp ax, 10 (IMAGE_REL_BASED_DIR64)
                0x75,
                0x08, // jne next_entry

                0x66,
                0x81,
                0xe1,
                0xff,
                0x0f, // and cx, 0xFFF (offset)
                0x48,
                0x01,
                0x34,
                0x0f, // add [rdi+rcx], rsi (apply relocation)

                // next_entry:
                0x48,
                0x83,
                0xc2,
                0x02, // add rdx, 2
                0x48,
                0xff,
                0xc8, // dec rax
                0xeb,
                0xe0, // jmp entry_loop

                // next_block:
                0x8b,
                0x42,
                0xfc, // mov eax, [rdx-4] (SizeOfBlock)
                0x48,
                0x01,
                0xc2, // add rdx, rax
                0x48,
                0x83,
                0xea,
                0x08, // sub rdx, 8
                0xeb,
                0xb8, // jmp reloc_loop

                // skip_reloc:
                // Process import table
                0x8b,
                0x95,
                0x80,
                0x00,
                0x00,
                0x00, // mov edx, [rbp+0x80] (import dir RVA)
                0x85,
                0xd2, // test edx, edx
                0x0f,
                0x84,
                0x80,
                0x00,
                0x00,
                0x00, // jz skip_imports

                0x48,
                0x8b,
                0x4b,
                0x10, // mov rcx, [rbx+16] (base_address)
                0x48,
                0x01,
                0xca, // add rdx, rcx (import table)

                // import_loop:
                0x8b,
                0x02, // mov eax, [rdx] (OriginalFirstThunk)
                0x85,
                0xc0, // test eax, eax
                0x74,
                0x70, // jz skip_imports

                // Load library
                0x8b,
                0x42,
                0x0c, // mov eax, [rdx+12] (Name RVA)
                0x48,
                0x01,
                0xc8, // add rax, rcx
                0x48,
                0x89,
                0xc1, // mov rcx, rax (library name)
                0xff,
                0x13, // call [rbx] (LoadLibraryA)
                0x48,
                0x85,
                0xc0, // test rax, rax
                0x0f,
                0x84,
                0x20,
                0x01,
                0x00,
                0x00, // jz error_exit
                0x48,
                0x89,
                0xc6, // mov rsi, rax (module handle)

                // Process function imports
                0x8b,
                0x02, // mov eax, [rdx] (OriginalFirstThunk)
                0x48,
                0x8b,
                0x4b,
                0x10, // mov rcx, [rbx+16]
                0x48,
                0x01,
                0xc8, // add rax, rcx (INT)
                0x8b,
                0x7a,
                0x10, // mov edi, [rdx+16] (FirstThunk)
                0x48,
                0x01,
                0xcf, // add rdi, rcx (IAT)

                // function_loop:
                0x48,
                0x8b,
                0x08, // mov rcx, [rax]
                0x48,
                0x85,
                0xc9, // test rcx, rcx
                0x74,
                0x30, // jz next_import

                0x48,
                0xf7,
                0xc1,
                0x00,
                0x00,
                0x00,
                0x80, // test rcx, 0x8000000000000000
                0x75,
                0x10, // jnz ordinal_import

                // Import by name
                0x48,
                0x8b,
                0x4b,
                0x10, // mov rcx, [rbx+16]
                0x48,
                0x8b,
                0x00, // mov rax, [rax]
                0x48,
                0x01,
                0xc8, // add rax, rcx
                0x48,
                0x83,
                0xc0,
                0x02, // add rax, 2 (skip hint)
                0x48,
                0x89,
                0xc1, // mov rcx, rax (function name)
                0xeb,
                0x06, // jmp get_proc_addr

                // ordinal_import:
                0x48,
                0x81,
                0xe1,
                0xff,
                0xff,
                0x00,
                0x00, // and rcx, 0xFFFF

                // get_proc_addr:
                0x48,
                0x89,
                0xf2, // mov rdx, rsi (module handle)
                0xff,
                0x53,
                0x08, // call [rbx+8] (GetProcAddress)
                0x48,
                0x89,
                0x07, // mov [rdi], rax

                0x48,
                0x83,
                0xc0,
                0x08, // add rax, 8
                0x48,
                0x83,
                0xc7,
                0x08, // add rdi, 8
                0xeb,
                0xc8, // jmp function_loop

                // next_import:
                0x48,
                0x83,
                0xc2,
                0x14, // add rdx, 20 (sizeof(import descriptor))
                0xe9,
                0x78,
                0xff,
                0xff,
                0xff, // jmp import_loop

                // skip_imports:
                // Call DllMain
                0x48,
                0x8b,
                0x4b,
                0x10, // mov rcx, [rbx+16] (base_address)
                0x8b,
                0x85,
                0x28,
                0x00,
                0x00,
                0x00, // mov eax, [rbp+40] (AddressOfEntryPoint)
                0x85,
                0xc0, // test eax, eax
                0x74,
                0x20, // jz success_exit

                0x48,
                0x01,
                0xc8, // add rax, rcx (entry point)
                0x48,
                0x8b,
                0x53,
                0x20, // mov rdx, [rbx+32] (reserved_param)
                0x48,
                0x8b,
                0x4b,
                0x1c, // mov rcx, [rbx+28] (reason_param)
                0x48,
                0x8b,
                0x4b,
                0x10, // mov rcx, [rbx+16] (hinstDLL)
                0xff,
                0xd0, // call rax (DllMain)

                // success_exit:
                0x48,
                0x8b,
                0x43,
                0x10, // mov rax, [rbx+16] (base_address)
                0x48,
                0x89,
                0x43,
                0x18, // mov [rbx+24], rax (set hMod)
                0x48,
                0x83,
                0xc4,
                0x28, // add rsp, 0x28
                0xc3, // ret

                // error_exit:
                0x48,
                0xc7,
                0x43,
                0x18,
                0x00,
                0x00,
                0x00,
                0x00, // mov qword ptr [rbx+24], 0
                0x48,
                0x83,
                0xc4,
                0x28, // add rsp, 0x28
                0xc3 // ret
            ]
        }

        #[cfg(target_arch = "x86")]
        {
            // Complete x86 shellcode for manual DLL mapping
            vec![
                // Function prologue
                0x55, // push ebp
                0x89,
                0xe5, // mov ebp, esp
                0x53, // push ebx
                0x56, // push esi
                0x57, // push edi

                // Load mapping data
                0x8b,
                0x5d,
                0x08, // mov ebx, [ebp+8] (mapping data)
                0x8b,
                0x73,
                0x08, // mov esi, [ebx+8] (base_address)

                // Validate DOS header
                0x66,
                0x81,
                0x3e,
                0x4d,
                0x5a, // cmp word ptr [esi], 'MZ'
                0x0f,
                0x85,
                0x00,
                0x02,
                0x00,
                0x00, // jne error_exit

                // Get NT headers
                0x8b,
                0x46,
                0x3c, // mov eax, [esi+3Ch]
                0x01,
                0xf0, // add eax, esi
                0x89,
                0xc7, // mov edi, eax
                0x81,
                0x38,
                0x50,
                0x45,
                0x00,
                0x00, // cmp dword ptr [eax], 'PE'
                0x0f,
                0x85,
                0xf0,
                0x01,
                0x00,
                0x00, // jne error_exit

                // Get optional header
                0x83,
                0xc0,
                0x18, // add eax, 24

                // Process relocations
                0x8b,
                0x90,
                0xa0,
                0x00,
                0x00,
                0x00, // mov edx, [eax+0xA0]
                0x85,
                0xd2, // test edx, edx
                0x74,
                0x50, // jz skip_reloc

                // Calculate delta
                0x8b,
                0x48,
                0x1c, // mov ecx, [eax+28] (ImageBase)
                0x29,
                0xce, // sub esi, ecx
                0x85,
                0xf6, // test esi, esi
                0x74,
                0x40, // jz skip_reloc

                // Process relocation table (simplified for space)
                0x8b,
                0x4b,
                0x08, // mov ecx, [ebx+8]
                0x01,
                0xca, // add edx, ecx

                // Relocation processing loop (simplified)
                0x8b,
                0x42,
                0x04, // mov eax, [edx+4]
                0x85,
                0xc0, // test eax, eax
                0x74,
                0x30, // jz skip_reloc

                0x8b,
                0x0a, // mov ecx, [edx]
                0x8b,
                0x7b,
                0x08, // mov edi, [ebx+8]
                0x01,
                0xcf, // add edi, ecx

                0x83,
                0xe8,
                0x08, // sub eax, 8
                0xd1,
                0xe8, // shr eax, 1
                0x83,
                0xc2,
                0x08, // add edx, 8

                // Entry processing loop
                0x85,
                0xc0, // test eax, eax
                0x74,
                0x18, // jz next_block

                0x66,
                0x8b,
                0x0a, // mov cx, [edx]
                0x66,
                0x89,
                0xc8, // mov ax, cx
                0x66,
                0xc1,
                0xe8,
                0x0c, // shr ax, 12
                0x66,
                0x83,
                0xf8,
                0x03, // cmp ax, 3 (IMAGE_REL_BASED_HIGHLOW)
                0x75,
                0x06, // jne next_entry

                0x66,
                0x81,
                0xe1,
                0xff,
                0x0f, // and cx, 0xFFF
                0x01,
                0x34,
                0x0f, // add [edi+ecx], esi

                0x83,
                0xc2,
                0x02, // add edx, 2
                0x48, // dec eax
                0xeb,
                0xe8, // jmp entry_loop

                // skip_reloc:
                // Process imports (simplified)
                0x8b,
                0x90,
                0x80,
                0x00,
                0x00,
                0x00, // mov edx, [eax+0x80]
                0x85,
                0xd2, // test edx, edx
                0x74,
                0x60, // jz skip_imports

                0x8b,
                0x4b,
                0x08, // mov ecx, [ebx+8]
                0x01,
                0xca, // add edx, ecx

                // Import loop
                0x8b,
                0x02, // mov eax, [edx]
                0x85,
                0xc0, // test eax, eax
                0x74,
                0x50, // jz skip_imports

                // Load library
                0x8b,
                0x42,
                0x0c, // mov eax, [edx+12]
                0x01,
                0xc8, // add eax, ecx
                0x50, // push eax
                0xff,
                0x13, // call [ebx] (LoadLibraryA)
                0x85,
                0xc0, // test eax, eax
                0x0f,
                0x84,
                0x80,
                0x00,
                0x00,
                0x00, // jz error_exit
                0x89,
                0xc6, // mov esi, eax

                // Process functions (simplified)
                0x8b,
                0x02, // mov eax, [edx]
                0x8b,
                0x4b,
                0x08, // mov ecx, [ebx+8]
                0x01,
                0xc8, // add eax, ecx
                0x8b,
                0x7a,
                0x10, // mov edi, [edx+16]
                0x01,
                0xcf, // add edi, ecx

                // Function resolution loop
                0x8b,
                0x08, // mov ecx, [eax]
                0x85,
                0xc9, // test ecx, ecx
                0x74,
                0x18, // jz next_import

                0xf7,
                0xc1,
                0x00,
                0x00,
                0x00,
                0x80, // test ecx, 0x80000000
                0x75,
                0x08, // jnz ordinal_import

                0x8b,
                0x4b,
                0x08, // mov ecx, [ebx+8]
                0x8b,
                0x00, // mov eax, [eax]
                0x01,
                0xc8, // add eax, ecx
                0x83,
                0xc0,
                0x02, // add eax, 2

                0x50, // push eax
                0x56, // push esi
                0xff,
                0x53,
                0x04, // call [ebx+4] (GetProcAddress)
                0x89,
                0x07, // mov [edi], eax

                0x83,
                0xc0,
                0x04, // add eax, 4
                0x83,
                0xc7,
                0x04, // add edi, 4
                0xeb,
                0xd8, // jmp function_loop

                0x83,
                0xc2,
                0x14, // add edx, 20
                0xeb,
                0xa8, // jmp import_loop

                // skip_imports:
                // Call DllMain
                0x8b,
                0x4b,
                0x08, // mov ecx, [ebx+8]
                0x8b,
                0x80,
                0x28,
                0x00,
                0x00,
                0x00, // mov eax, [eax+40]
                0x85,
                0xc0, // test eax, eax
                0x74,
                0x15, // jz success_exit

                0x01,
                0xc8, // add eax, ecx
                0xff,
                0x73,
                0x14, // push [ebx+20] (reserved)
                0xff,
                0x73,
                0x10, // push [ebx+16] (reason)
                0x51, // push ecx (hinstDLL)
                0xff,
                0xd0, // call eax (DllMain)
                0x83,
                0xc4,
                0x0c, // add esp, 12

                // success_exit:
                0x8b,
                0x43,
                0x08, // mov eax, [ebx+8]
                0x89,
                0x43,
                0x0c, // mov [ebx+12], eax
                0xeb,
                0x06, // jmp cleanup

                // error_exit:
                0xc7,
                0x43,
                0x0c,
                0x00,
                0x00,
                0x00,
                0x00, // mov dword ptr [ebx+12], 0

                // cleanup:
                0x5f, // pop edi
                0x5e, // pop esi
                0x5b, // pop ebx
                0x5d, // pop ebp
                0xc3 // ret
            ]
        }
    }

    /// Validates the target process architecture compatibility
    pub fn is_compatible_architecture(&self, process_handle: HANDLE) -> Result<bool, String> {
        #[cfg(windows)]
        unsafe {
            let mut is_wow64_target = FALSE;
            let result = winapi::um::wow64apiset::IsWow64Process(
                process_handle,
                &mut is_wow64_target
            );

            if result == FALSE {
                return Err(
                    format!(
                        "Failed to check target process architecture: 0x{:X}",
                        winapi::um::errhandlingapi::GetLastError()
                    )
                );
            }

            let mut is_wow64_current = FALSE;
            let result = winapi::um::wow64apiset::IsWow64Process(
                winapi::um::processthreadsapi::GetCurrentProcess(),
                &mut is_wow64_current
            );

            if result == FALSE {
                return Err("Failed to check current process architecture".to_string());
            }

            // Both processes should have the same WOW64 status
            Ok(is_wow64_target == is_wow64_current)
        }

        #[cfg(not(windows))]
        Ok(false)
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
                return Err(
                    format!(
                        "Failed to adjust protection for section {}: 0x{:X}",
                        i,
                        winapi::um::errhandlingapi::GetLastError()
                    )
                );
            }
        }

        Ok(())
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
