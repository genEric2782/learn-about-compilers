use std::{path::PathBuf, process::Command};

fn main() {
    println!("cargo:rustc-link-search=native=./dlibs");

    // C#
    println!("cargo:rustc-link-search=native=../learn-to-compile-parser-c#/bin/Release/net8.0/linux-x64/publish");
    println!("cargo:rustc-link-lib=dylib=learn-to-compile-c-sharp");

    // Haskell
    println!("cargo:rustc-link-lib=dylib=haskellTypeChecker");
    // Foor haskell runtime
    println!("cargo:rustc-link-arg=-Wl,-rpath,./libs");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/home/generic/.ghcup/ghc/9.10.3/lib/ghc-9.10.3/lib/x86_64-linux-ghc-9.10.3");

    println!("cargo:rustc-link-search=native=/home/generic/.ghcup/ghc/9.10.3/lib/ghc-9.10.3/lib/x86_64-linux-ghc-9.10.3");
    println!("cargo:rustc-link-lib=dylib=HSrts-1.0.2-ghc9.10.3");

    // Scala
    let scala_lib = "/home/generic/compiler_project/take2/learn-about-compilers/learn-to-compile-IR-tac-scala/target/native-image/";
    println!("cargo:rustc-link-search=native={scala_lib}");
    println!("cargo:rustc-link-lib=dylib=learn-about-compilers-scala");

    // Zig 
    let zig_dir = PathBuf::from("../learn-to-compile-mapTacAsm-zig");

    let status = Command::new("zig")
        .args(["build", "-Doptimize=ReleaseSafe"])
        .current_dir(&zig_dir)
        .status()
        .expect("zig build failed — is Zig installed?");
    assert!(status.success());

    let lib_path = zig_dir.join("zig-out/lib");
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=dylib=tac_codegen");
    println!("cargo:rerun-if-changed=../learn-to-compile-mapTacAsm-zig/src/ffi.zig");
    println!("cargo:rerun-if-changed=../learn-to-compile-mapTacAsm-zig/src/asmMap.zig");
 
}