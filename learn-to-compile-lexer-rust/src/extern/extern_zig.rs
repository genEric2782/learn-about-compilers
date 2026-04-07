use std::sync::Once;

static INIT: Once = Once::new();

#[repr(C)]
struct AsmResult {
    ptr: *mut u8,
    len: usize,
    err: bool,
}

unsafe extern "C" {
    unsafe fn tac_init();
    unsafe fn tac_compile(json_ptr: *const u8, json_len: usize) -> AsmResult;
    unsafe fn tac_free_result(result: AsmResult);
}

/// Compile a TAC JSON string into assembly lines.
/// Returns Ok(String) on success, Err(String) with the Zig error name on failure.
pub fn compile(json: &str) -> Result<String, String> {
    // tac_init is idempotent on the Rust side thanks to Once,
    // but the Zig side must only push registers once or the stack fills up with duplicates.
    INIT.call_once(|| {
        unsafe { tac_init() };
    });

    let result = unsafe {
        tac_compile(json.as_ptr(), json.len())
    };

    if result.ptr.is_null() || result.len == 0 {
        return Err(String::from("empty result from Zig"));
    }

    let text = unsafe {
        let bytes = std::slice::from_raw_parts(result.ptr, result.len);
        String::from_utf8_lossy(bytes).into_owned()
    };

    let ok = !result.err;
    unsafe { tac_free_result(result) };

    if ok { Ok(text) } else { Err(text) }
}