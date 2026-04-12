use std::ffi::CString;
use std::os::raw::c_char;

// Just Run the VM
// unsafe extern "C" {
//     pub unsafe fn ReadTacFromFfi(input: *const c_char);
// }

unsafe extern "C" {
    pub unsafe fn SerializeFfi(input: *const c_char, output_path: *const c_char);
}

unsafe extern "C" {
    pub unsafe fn DeserializeFfi(input_path: *const c_char);
}