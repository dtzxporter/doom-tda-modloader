use std::mem::transmute;

use widestring::U16CString;

use windows_sys::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS64;
use windows_sys::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::SystemServices::IMAGE_DOS_HEADER;

/// Gets the main module path.
pub fn main_module_path() -> String {
    let mut file_name: [u16; 256] = [0; 256];

    let length = unsafe { GetModuleFileNameW(std::ptr::null_mut(), file_name.as_mut_ptr(), 256) };

    unsafe { U16CString::from_ptr(file_name.as_ptr(), length as usize) }
        .expect("Failed to get main module path!")
        .to_string_lossy()
}

/// Gets the main module .text segment as a slice of bytes.
pub fn main_module() -> &'static mut [u8] {
    let main_module = unsafe { GetModuleHandleW(std::ptr::null()) };
    let dos_header: *const IMAGE_DOS_HEADER = unsafe { transmute(main_module) };
    let nt_header: *const IMAGE_NT_HEADERS64 = unsafe {
        transmute::<_, *const IMAGE_NT_HEADERS64>(main_module.add((*dos_header).e_lfanew as usize))
    };

    unsafe {
        std::slice::from_raw_parts_mut(
            main_module as *mut _,
            (*nt_header).OptionalHeader.SizeOfCode as usize,
        )
    }
}
