use crate::bindings::*;

#[cfg(feature = "info_box")]
const LEN_LIMIT: c_int = 24;

#[cfg(feature = "info_box")]
const NAME_SIZE: usize = size_of::<CHAR>() * (LEN_LIMIT as usize) + 1;

#[cfg(feature = "info_box")]
lazy_static! {
    // Earlier pointer to a buffer was passed by "lparam", but it was "unstable?"
    // ToDo: drop usage of lazy_static
    static ref BUFFER: Mutex<String> = {
        Mutex::new(String::new())
    };
}

#[allow(unused_variables)]
unsafe extern "system" fn callback_func(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let id = GetCurrentProcessId();
    let other_id = malloc(size_of::<DWORD>()) as *mut DWORD;

    // WTF?! Without this print crash occurs.
    // ToDo: research
    println!("{:?} - {}", hwnd, id);
    GetWindowThreadProcessId(hwnd, other_id);

    if !other_id.is_null() && id == *other_id {
        #[cfg(feature = "info_box")]
        {
            if lparam == 0 {
                let mut local_buffer = format!("[{:?}]", hwnd);
                let window_name_ptr = malloc(NAME_SIZE) as LPSTR;

                let window_name_len = GetWindowTextA(hwnd, window_name_ptr, LEN_LIMIT + 1);
                if window_name_len != 0 {
                    let window_name = CStr::from_ptr(window_name_ptr).to_string_lossy();

                    let window_name = if window_name_len == LEN_LIMIT {
                        format!("{}...", window_name)
                    } else {
                        format!("{}", window_name)
                    };

                    local_buffer = format!("{} ({})", local_buffer, window_name);
                };

                // I am not sure about this one
                dealloc(
                    window_name_ptr as *mut _,
                    Layout::from_size_align_unchecked(NAME_SIZE, 1),
                );

                local_buffer.push_str(": ");

                let msg = if SetWindowDisplayAffinity(hwnd, 0x00000011) == FALSE {
                    format!("Error 0x{:X}", GetLastError())
                } else {
                    format!("Success")
                };

                local_buffer = format!("{}{}\n", local_buffer, msg);

                if let Ok(mut guard) = BUFFER.lock() {
                    guard.extend(local_buffer.chars().into_iter());
                }
            } else {
                SetWindowDisplayAffinity(hwnd, 0x00000011);
            }
        }

        #[cfg(not(feature = "info_box"))]
        SetWindowDisplayAffinity(hwnd, 0x00000011);
    }

    // I am not sure about this one
    dealloc(
        other_id as *mut _,
        Layout::from_size_align_unchecked(size_of::<DWORD>(), 1),
    );

    TRUE
}

pub unsafe fn run() {
    #[cfg(feature = "info_box")]
    {
        EnumWindows(Some(callback_func), 0);

        let tittle = CString::new("Info").unwrap();

        let mut buffer = if let Ok(mut guard) = BUFFER.lock() {
            std::mem::take(&mut *guard)
        } else {
            String::new()
        };

        if !buffer.is_empty() {
            buffer.pop();
            let buffer = CString::new(buffer).unwrap();
            MessageBoxA(null_mut(), buffer.as_ptr(), tittle.as_ptr(), MB_OK);
        } else {
            let msg = CString::new("Did not find any valid windows!").unwrap();
            MessageBoxA(null_mut(), msg.as_ptr(), tittle.as_ptr(), MB_ICONERROR);
        }
    }

    // Main loop
    loop {
        EnumWindows(Some(callback_func), 1);
        sleep(Duration::from_millis(10));
    }
}
