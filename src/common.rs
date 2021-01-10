
use std::io::Error;

pub use winapi::shared::minwindef::{UINT, DWORD};
pub use winapi::shared::windef::{HWND};
pub use std::ptr::null_mut as NULL;
pub type SysResult<T> = Result<T, Error>;

// #[allow(unused)]
#[macro_export]
macro_rules! error_if_zero {
    ($x:expr) => {
        {
            let tmp = $x;
            match { tmp } {
                0 => Err(std::io::Error::last_os_error()),
                _ => Ok(tmp)
            }
        }
    };
    ($return_what:expr, $x:expr) => {
        {
            match { $x } {
                0 => Err(std::io::Error::last_os_error()),
                _ => Ok($return_what)
            }
        }
    }
}
