use std::ffi::c_void;

use windows::{
    core::PWSTR,
    imp::{
        FormatMessageW, GetLastError, FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM,
        FORMAT_MESSAGE_IGNORE_INSERTS,
    },
};

pub fn last_error_message() -> String {
    let message_buffer = PWSTR::null();
    let void = std::ptr::null() as *const c_void;
    let iptr = std::ptr::null() as *const *const i8;

    unsafe {
        FormatMessageW(
            FORMAT_MESSAGE_ALLOCATE_BUFFER
                | FORMAT_MESSAGE_FROM_SYSTEM
                | FORMAT_MESSAGE_IGNORE_INSERTS,
            void,
            GetLastError(),
            0,
            message_buffer,
            0,
            iptr,
        );

        message_buffer.to_string().unwrap()
    }
}
