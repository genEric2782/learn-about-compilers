use std::os::raw::{c_char, c_int};

// #[link(name = "haskellTypeChecker")]
unsafe extern "C" {
    // Haskell init functions to load the haskell runtime 
    pub unsafe fn hs_init(argc: *mut i32, argv: *mut *mut *mut i8);
    pub unsafe fn hs_exit();
    pub unsafe fn readAST(input: *const c_char) -> c_int; // bool
}
