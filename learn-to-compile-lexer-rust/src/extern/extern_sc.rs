//! Raw FFI bindings to the GraalVM-compiled Scala library.
//! GraalVM Native Image shared libs require you to create an "isolate"
//! (a lightweight JVM-like context) before calling any entry points.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

// ----- GraalVM isolate types (from graal_isolate.h) -----

// Match exactly what graal_isolate.h defines
#[repr(C)]
pub struct GraalIsolate(u8);
#[repr(C)]
pub struct GraalIsolateThread(u8);

pub type GraalIsolatePtr       = *mut GraalIsolate;
pub type GraalIsolateThreadPtr = *mut GraalIsolateThread;

// #[repr(C)]
// pub struct GraalCreateIsolateParams {
//     pub version: c_int,
//     // ... more fields exist but zeroing them is fine for defaults
//     _reserved: [u8; 64],
// }

// impl Default for GraalCreateIsolateParams {
//     fn default() -> Self {
//         // Safety: all-zero is valid for the default params struct
//         unsafe { std::mem::zeroed() }
//     }
// }

unsafe extern "C" {
    // GraalVM isolate lifecycle — provided by the native image runtime
    pub unsafe fn graal_create_isolate(
        params: *mut std::ffi::c_void,
        isolate: *mut GraalIsolatePtr,
        thread:  *mut GraalIsolateThreadPtr,
    ) -> i32;

    pub unsafe fn graal_tear_down_isolate(thread: GraalIsolateThreadPtr) -> i32;

    // Our Scala entry points
    pub unsafe fn process_ast(
        thread:   GraalIsolateThreadPtr,
        json_ptr: *const c_char,
    ) -> *mut c_char;

    // pub unsafe fn free_string(
    //     thread: GraalIsolateThreadPtr,
    //     ptr:    *mut c_char,
    // );
}

// ----- Safe wrapper -----

pub struct ScalaAstProcessor {
    thread:  GraalIsolateThreadPtr,
}

impl ScalaAstProcessor {
    pub fn new() -> Result<Self, String> {
        let mut isolate: GraalIsolatePtr = std::ptr::null_mut();
        let mut thread:  GraalIsolateThreadPtr = std::ptr::null_mut();

        let rc = unsafe {
            graal_create_isolate(std::ptr::null_mut(), &mut isolate, &mut thread)
        };

        if rc != 0 {
            return Err(format!("graal_create_isolate failed with code {rc}"));
        }
        Ok(Self { thread })
    }

    /// Send a JSON AST string to Scala and get a JSON result back.
    pub fn process(&self, ast_json: &str) -> Result<String, String> {
        let c_input = CString::new(ast_json)
            .map_err(|e| format!("CString error: {e}"))?;

        let raw_out = unsafe {
            process_ast(self.thread, c_input.as_ptr())
        };

        if raw_out.is_null() {
            return Err("process_ast returned null".into());
        }

        // Copy the result out of native memory before freeing
        let result = unsafe { CStr::from_ptr(raw_out) }
            .to_str()
            .map(|s| s.to_owned())
            .map_err(|e| format!("UTF-8 error: {e}"));

        // unsafe { free_string(self.thread, raw_out) };
        result
    }
}

impl Drop for ScalaAstProcessor {
    fn drop(&mut self) {
        unsafe { graal_tear_down_isolate(self.thread) };
    }
}