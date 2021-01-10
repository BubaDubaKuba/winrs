

pub fn get_last_error() -> u32 {
    use winapi::um::errhandlingapi::GetLastError;
    unsafe { GetLastError() }
}

pub fn set_last_error(errcode: u32) {
    use winapi::um::errhandlingapi::SetLastError;
    unsafe { SetLastError(errcode) }
}

