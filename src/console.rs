
use winapi::um::consoleapi;
use winapi::um::wincon;
use crate::{error_if_zero, SysResult};

// use winapi::shared::ntdef::HANDLE;

pub fn get_console_output_cp() -> SysResult<u32> {
    use consoleapi::GetConsoleOutputCP;
    error_if_zero!(unsafe { GetConsoleOutputCP() })
}

pub fn set_console_output_cp(code_page: u32) -> SysResult<()> {
    use wincon::SetConsoleOutputCP;
    error_if_zero!((), unsafe { SetConsoleOutputCP(code_page) })
}

