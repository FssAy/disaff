#[cfg(not(target_os = "windows"))]
compile_error!("Disaff can be used only on Windows operation system!");

#[cfg(feature = "info_box")]
#[macro_use]
extern crate lazy_static;

mod bindings;
mod core;

use bindings::*;

unsafe extern "system" fn my_thread_function(_lp_param: LPVOID) -> DWORD {
    core::run();
    0
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(
    hinst_dll: HINSTANCE,
    fdw_reason: DWORD,
    lpv_reserved: LPVOID,
) -> BOOL {
    match fdw_reason {
        DLL_PROCESS_ATTACH => unsafe {
            DisableThreadLibraryCalls(hinst_dll);
            CreateThread(
                null_mut(),
                0,
                Some(my_thread_function),
                hinst_dll as LPVOID,
                0,
                null_mut(),
            );
        },

        DLL_PROCESS_DETACH => if !lpv_reserved.is_null() {},

        _ => {}
    }

    TRUE
}
