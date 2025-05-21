mod error;
mod hooks;
mod test;
mod utilities;
mod version;

use error::*;
use hooks::*;

use std::ffi::c_void;

use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::System::LibraryLoader::*;
use windows_sys::Win32::System::SystemServices::*;
use windows_sys::Win32::System::Threading::CreateThread;

/// Initialization routine, outside of dllmain so that we can debug it.
extern "system" fn initialize(_lp_param: *mut c_void) -> u32 {
    let main_module = utilities::main_module_path();

    if !main_module.contains("DOOMTheDarkAges.exe") {
        return 0;
    }

    #[cfg(debug_assertions)]
    {
        use windows_sys::Win32::System::Console::AllocConsole;
        use windows_sys::Win32::System::Threading::Sleep;

        unsafe { Sleep(2000) };
        unsafe { AllocConsole() };
    }

    println!("{:?}", main_module);

    #[cfg(debug_assertions)]
    println!(
        "Initializing maykr v{}.{}{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH")
    );

    match install_hooks() {
        Ok(()) => {
            #[cfg(debug_assertions)]
            println!("Maykr initialize success");
        }
        Err(e) => {
            #[cfg(not(debug_assertions))]
            {
                use std::ffi::CString;

                use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetActiveWindow;
                use windows_sys::Win32::UI::WindowsAndMessaging::*;

                let text = CString::new(format!(
                    "Failed to initialize maykr: {:#02x}. Please report this!",
                    e.discriminant()
                ))
                .expect("Failed to create string!");

                let title = format!(
                    "Daisy v{}.{}{}",
                    env!("CARGO_PKG_VERSION_MAJOR"),
                    env!("CARGO_PKG_VERSION_MINOR"),
                    env!("CARGO_PKG_VERSION_PATCH")
                );

                let title = CString::new(title).expect("Failed to create string!");

                unsafe {
                    MessageBoxA(
                        GetActiveWindow(),
                        text.as_ptr() as _,
                        title.as_ptr() as _,
                        MB_OK | MB_ICONWARNING,
                    )
                };
            }

            #[cfg(debug_assertions)]
            println!("Failed to initialize maykr: {:?}", e);
        }
    }

    0
}

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(
    hmodule: HINSTANCE,
    ul_reason_for_call: u32,
    _lp_reserved: *mut c_void,
) -> bool {
    match ul_reason_for_call {
        DLL_PROCESS_ATTACH => {
            DisableThreadLibraryCalls(hmodule);

            CreateThread(
                std::ptr::null(),
                0,
                Some(initialize),
                std::ptr::null(),
                0,
                std::ptr::null_mut(),
            );
        }
        _ => {
            // Nothing.
        }
    }

    true
}
