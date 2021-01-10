
use winapi::um::winuser;
use std::io::Error;
use std::mem::MaybeUninit;

use crate::error_if_zero;
use crate::{SysResult, HWND};


pub fn open_clipboard(hwnd: HWND) -> SysResult<()> {
    use winuser::OpenClipboard;
    error_if_zero!((), unsafe { OpenClipboard(hwnd) })
}

pub fn close_clipboard() -> SysResult<()> {
    use winuser::CloseClipboard;
    error_if_zero!((), unsafe { CloseClipboard() })
}

pub fn enum_clipboard_formats(format: u32) -> SysResult<u32> {
    use winuser::EnumClipboardFormats;
    use crate::error::get_last_error;

    let new_format = unsafe { EnumClipboardFormats(format) };
    match { new_format } {
        0 => {
            // If last error is 0 then it means that enum is done.
            // If it's not 0, then something bad happened.
            if get_last_error() == 0 {
                Ok(0)
            } else {
                Err(Error::last_os_error())
            }
        },
        _ => Ok(new_format)
    }
}

pub fn get_clipboard_format_name(format: u32) -> SysResult<String> {
    use winuser::GetClipboardFormatNameW;
    let mut buf: [u16; 256] = unsafe { MaybeUninit::uninit().assume_init() };
    let length = unsafe { GetClipboardFormatNameW(format, buf.as_mut_ptr(), 256) } as usize;
    println!("got {}", length);
    match { length } {
        0 => Ok("???".to_string()),
        _ => Ok(String::from_utf16_lossy(&buf[0..length]))
    }
}

