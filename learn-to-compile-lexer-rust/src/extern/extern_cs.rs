use std::ffi::{CString, CStr};
use std::os::raw::c_char;
// use crate::rslexer::lexer::{TokenKind}; //Token,

// #[link(name = "liblearn-to-compile-c-sharp")]
// For the Rust lexer to effectivly communicate with the C# parser
// They communnicate thorugh C functions 
// the C# .so library is in target/debug and has the functions below 
// all external code must be marke dunsafe
unsafe extern "C" {
    unsafe fn parse_to_json(input: *const c_char) -> *mut c_char;
    unsafe fn free_string(ptr: *mut c_char);
}

pub fn call_parser(input: &CString) -> String {
    // let c_input = CString::new(input).unwrap();

    unsafe {
        let result_ptr = parse_to_json(input.as_ptr());
        let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();

        free_string(result_ptr);

        result
    }
}

