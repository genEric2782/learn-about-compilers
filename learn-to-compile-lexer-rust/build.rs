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

    // C++ 
    let out_dir = PathBuf::from("../learn-to-compile-elfgen-c++")
        .canonicalize()
        .expect("Could not resolve c++ project path");

    let sources = [
        "parseAsm",
        "byteGeneration", // whatever the file with generateInstructionBytes is called
        "elf",
    ];

    let mut obj_files: Vec<PathBuf> = Vec::new();    // Compile each .cpp to a .o
    for src in &sources {
        let cpp_file = out_dir.join(format!("{src}.cpp"));
        let obj_file = out_dir.join(format!("{src}.o"));

        let status = Command::new("g++")
            .args(["-O2", "-c"])
            .arg(&cpp_file)
            .arg("-o")
            .arg(&obj_file)
            .status()
            .expect("Failed to invoke g++");

        assert!(status.success(), "C++ compilation failed for {src}.cpp");

        obj_files.push(obj_file);

        println!("cargo:rerun-if-changed={}", cpp_file.display());
    }

    // Bundle all .o files into one .a
    let lib_out = out_dir.join("libparseAsm.a");
    let status = Command::new("ar")
        .arg("rcs")
        .arg(&lib_out)
        .args(&obj_files)
        .status()
        .expect("Failed to create static lib");

    assert!(status.success(), "ar failed");

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=parseAsm");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // GO
    println!("cargo:rustc-link-search=native=../learn-to-compile-bytecode-go");
    println!("cargo:rustc-link-lib=dylib=tacbytecode");
}
