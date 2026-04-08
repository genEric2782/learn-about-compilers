use std::ffi::CString;
use std::os::raw::{c_char, c_int};

unsafe extern "C" {
    unsafe fn compile_asm_to_elf(asm_src: *const c_char, output_path: *const c_char) -> bool;
}

pub fn compile_from_asm(source: &str, output_path: &str) -> Result<(), String> {
    let c_src  = CString::new(source).map_err(|e| e.to_string())?;
    let c_path = CString::new(output_path).map_err(|e| e.to_string())?;

    let ok = unsafe { compile_asm_to_elf(c_src.as_ptr(), c_path.as_ptr()) };

    if ok {
        Ok(())
    } else {
        Err(format!("C++ failed to compile assembly to ELF at '{output_path}'"))
    }
}
