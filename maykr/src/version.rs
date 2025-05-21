use std::ffi::c_void;
use std::ffi::CString;
use std::path::Path;

use widestring::U16CString;

use windows_sys::Win32::System::LibraryLoader::*;
use windows_sys::Win32::System::SystemInformation::GetSystemDirectoryW;

#[no_mangle]
extern "system" fn VerQueryValueA(
    pblock: *mut c_void,
    subblock: *mut c_void,
    lpbuffer: *mut c_void,
    pulength: *mut c_void,
) -> i64 {
    let mut file_name: [u16; 256] = [0; 256];

    let length = unsafe { GetSystemDirectoryW(file_name.as_mut_ptr(), 256) };

    let system = unsafe { U16CString::from_ptr(file_name.as_ptr(), length as usize) }
        .expect("Failed to get system path!")
        .to_string_lossy()
        .to_string();

    let path = Path::new(&system).join("version.dll");
    let path_name = U16CString::from_os_str(path.as_os_str()).expect("Failed to get path name!");

    let proc = CString::new("VerQueryValueA").expect("Failed to get proc!");

    let original = unsafe { LoadLibraryW(path_name.as_ptr()) };
    let address = unsafe { GetProcAddress(original, proc.as_ptr() as _) };

    let function = unsafe {
        std::mem::transmute::<
            unsafe extern "system" fn() -> isize,
            extern "system" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void) -> i64,
        >(address.unwrap())
    };

    function(pblock, subblock, lpbuffer, pulength)
}

#[no_mangle]
extern "system" fn VerQueryValueW(
    pblock: *mut c_void,
    subblock: *mut c_void,
    lpbuffer: *mut c_void,
    pulength: *mut c_void,
) -> i64 {
    let mut file_name: [u16; 256] = [0; 256];

    let length = unsafe { GetSystemDirectoryW(file_name.as_mut_ptr(), 256) };

    let system = unsafe { U16CString::from_ptr(file_name.as_ptr(), length as usize) }
        .expect("Failed to get system path!")
        .to_string_lossy()
        .to_string();

    let path = Path::new(&system).join("version.dll");
    let path_name = U16CString::from_os_str(path.as_os_str()).expect("Failed to get path name!");

    let proc = CString::new("VerQueryValueW").expect("Failed to get proc!");

    let original = unsafe { LoadLibraryW(path_name.as_ptr()) };
    let address = unsafe { GetProcAddress(original, proc.as_ptr() as _) };

    let function = unsafe {
        std::mem::transmute::<
            unsafe extern "system" fn() -> isize,
            extern "system" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void) -> i64,
        >(address.unwrap())
    };

    function(pblock, subblock, lpbuffer, pulength)
}

#[no_mangle]
extern "system" fn GetFileVersionInfoA(
    filename: *mut c_void,
    handle: u32,
    dwlength: u32,
    lpdata: *mut c_void,
) -> i64 {
    let mut file_name: [u16; 256] = [0; 256];

    let length = unsafe { GetSystemDirectoryW(file_name.as_mut_ptr(), 256) };

    let system = unsafe { U16CString::from_ptr(file_name.as_ptr(), length as usize) }
        .expect("Failed to get system path!")
        .to_string_lossy()
        .to_string();

    let path = Path::new(&system).join("version.dll");
    let path_name = U16CString::from_os_str(path.as_os_str()).expect("Failed to get path name!");

    let proc = CString::new("GetFileVersionInfoA").expect("Failed to get proc!");

    let original = unsafe { LoadLibraryW(path_name.as_ptr()) };
    let address = unsafe { GetProcAddress(original, proc.as_ptr() as _) };

    let function = unsafe {
        std::mem::transmute::<
            unsafe extern "system" fn() -> isize,
            extern "system" fn(*mut c_void, u32, u32, *mut c_void) -> i64,
        >(address.unwrap())
    };

    function(filename, handle, dwlength, lpdata)
}

#[no_mangle]
extern "system" fn GetFileVersionInfoW(
    filename: *mut c_void,
    handle: u32,
    dwlength: u32,
    lpdata: *mut c_void,
) -> i64 {
    let mut file_name: [u16; 256] = [0; 256];

    let length = unsafe { GetSystemDirectoryW(file_name.as_mut_ptr(), 256) };

    let system = unsafe { U16CString::from_ptr(file_name.as_ptr(), length as usize) }
        .expect("Failed to get system path!")
        .to_string_lossy()
        .to_string();

    let path = Path::new(&system).join("version.dll");
    let path_name = U16CString::from_os_str(path.as_os_str()).expect("Failed to get path name!");

    let proc = CString::new("GetFileVersionInfoW").expect("Failed to get proc!");

    let original = unsafe { LoadLibraryW(path_name.as_ptr()) };
    let address = unsafe { GetProcAddress(original, proc.as_ptr() as _) };

    let function = unsafe {
        std::mem::transmute::<
            unsafe extern "system" fn() -> isize,
            extern "system" fn(*mut c_void, u32, u32, *mut c_void) -> i64,
        >(address.unwrap())
    };

    function(filename, handle, dwlength, lpdata)
}

#[no_mangle]
extern "system" fn GetFileVersionInfoSizeA(filename: *mut c_void, lphandle: *mut c_void) -> i64 {
    let mut file_name: [u16; 256] = [0; 256];

    let length = unsafe { GetSystemDirectoryW(file_name.as_mut_ptr(), 256) };

    let system = unsafe { U16CString::from_ptr(file_name.as_ptr(), length as usize) }
        .expect("Failed to get system path!")
        .to_string_lossy()
        .to_string();

    let path = Path::new(&system).join("version.dll");
    let path_name = U16CString::from_os_str(path.as_os_str()).expect("Failed to get path name!");

    let proc = CString::new("GetFileVersionInfoSizeA").expect("Failed to get proc!");

    let original = unsafe { LoadLibraryW(path_name.as_ptr()) };
    let address = unsafe { GetProcAddress(original, proc.as_ptr() as _) };

    let function = unsafe {
        std::mem::transmute::<
            unsafe extern "system" fn() -> isize,
            extern "system" fn(*mut c_void, *mut c_void) -> i64,
        >(address.unwrap())
    };

    function(filename, lphandle)
}

#[no_mangle]
extern "system" fn GetFileVersionInfoSizeW(filename: *mut c_void, lphandle: *mut c_void) -> i64 {
    let mut file_name: [u16; 256] = [0; 256];

    let length = unsafe { GetSystemDirectoryW(file_name.as_mut_ptr(), 256) };

    let system = unsafe { U16CString::from_ptr(file_name.as_ptr(), length as usize) }
        .expect("Failed to get system path!")
        .to_string_lossy()
        .to_string();

    let path = Path::new(&system).join("version.dll");
    let path_name = U16CString::from_os_str(path.as_os_str()).expect("Failed to get path name!");

    let proc = CString::new("GetFileVersionInfoSizeW").expect("Failed to get proc!");

    let original = unsafe { LoadLibraryW(path_name.as_ptr()) };
    let address = unsafe { GetProcAddress(original, proc.as_ptr() as _) };

    let function = unsafe {
        std::mem::transmute::<
            unsafe extern "system" fn() -> isize,
            extern "system" fn(*mut c_void, *mut c_void) -> i64,
        >(address.unwrap())
    };

    function(filename, lphandle)
}
