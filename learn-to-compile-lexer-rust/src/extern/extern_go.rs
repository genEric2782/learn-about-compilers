use std::ffi::CString;
use std::os::raw::c_char;

unsafe extern "C" {
    pub unsafe fn ReadTacFromFfi(input: *const c_char);
}