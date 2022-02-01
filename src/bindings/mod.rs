#[cfg(feature = "info_box")]
mod info_box;

#[cfg(feature = "info_box")]
pub use info_box::*;

pub use libc::malloc;
pub use std::alloc::{dealloc, Layout};
pub use std::mem::size_of;
pub use std::ptr::null_mut;
pub use std::thread::sleep;
pub use std::time::Duration;
pub use winapi::ctypes::c_int;
pub use winapi::shared::minwindef::{BOOL, DWORD, FALSE, HINSTANCE, LPARAM, LPVOID, TRUE};
pub use winapi::shared::windef::HWND;
pub use winapi::um::libloaderapi::{DisableThreadLibraryCalls, FreeLibraryAndExitThread};
pub use winapi::um::processthreadsapi::GetCurrentProcessId;
pub use winapi::um::processthreadsapi::{CreateThread, GetCurrentProcess};
pub use winapi::um::winnt::{CHAR, LPSTR};
pub use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
pub use winapi::um::winuser::{EnumWindows, GetWindowThreadProcessId, SetWindowDisplayAffinity};
