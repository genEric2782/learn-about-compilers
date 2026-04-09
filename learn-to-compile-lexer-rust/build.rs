use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

// ── helpers ────────────────────────────────────────────────────────────────

fn run(cmd: &mut Command, context: &str) {
    let status = cmd.status().unwrap_or_else(|e| panic!("{context}: failed to spawn — {e}"));
    assert!(status.success(), "{context}: command exited with {status}");
}

fn copy_to_libs(src: &Path, libs_dir: &Path) {
    let name = src.file_name().unwrap_or_else(|| panic!("no filename in {}", src.display()));
    let dst = libs_dir.join(name);
    fs::copy(src, &dst)
        .unwrap_or_else(|e| panic!("copy {} → {} failed: {e}", src.display(), dst.display()));
    println!("cargo:warning=copied {} → {}", src.display(), dst.display());
}

/// Root of the whole multi-language project  (parent of the rust crate)
fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("rust crate has no parent dir")
        .to_path_buf()
}

// ── change detection ───────────────────────────────────────────────────────

/// Emit rerun-if-changed for every file under a directory with a given extension.
/// Falls back to watching the dir itself if it can't be read.
fn watch_dir(dir: &Path, ext: &str) {
    let Ok(entries) = fs::read_dir(dir) else {
        println!("cargo:rerun-if-changed={}", dir.display());
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            watch_dir(&path, ext);
        } else if path.extension().map_or(false, |e| e == ext) {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}

// ── build switches ─────────────────────────────────────────────────────────

#[derive(Debug)]
struct BuildFlags {
    csharp:  bool,
    haskell: bool,
    scala:   bool,
    go:      bool,
    zig:     bool,
    cpp:     bool,
}

impl BuildFlags {
    fn from_env() -> Self {
        // REBUILD=all          → rebuild everything
        // REBUILD=none         → skip all native builds (just relink)
        // REBUILD=csharp,go    → rebuild only those languages
        // unset                → default: only rebuild if sources changed
        //                        (cargo handles this via rerun-if-changed)
        let var = std::env::var("REBUILD").unwrap_or_default();
        let var = var.trim().to_lowercase();

        if var == "all" {
            return Self { csharp: true, haskell: true, scala: true, go: true, zig: true, cpp: true };
        }
        if var == "none" || var == "skip" {
            return Self { csharp: false, haskell: false, scala: false, go: false, zig: false, cpp: false };
        }

        // Comma-separated list of languages to rebuild
        if !var.is_empty() {
            let langs: Vec<&str> = var.split(',').map(str::trim).collect();
            return Self {
                csharp:  langs.contains(&"csharp")  || langs.contains(&"c#"),
                haskell: langs.contains(&"haskell"),
                scala:   langs.contains(&"scala"),
                go:      langs.contains(&"go"),
                zig:     langs.contains(&"zig"),
                cpp:     langs.contains(&"cpp")     || langs.contains(&"c++"),
            };
        }

        // Default: let cargo's rerun-if-changed decide.
        // We still run the build — but each function only does real work
        // when cargo determines sources changed (which it checks via the
        // rerun-if-changed directives we emit). On a cold build everything runs.
        Self { csharp: true, haskell: true, scala: true, go: true, zig: true, cpp: true }
    }
}

// ── per-language builders ──────────────────────────────────────────────────

fn build_csharp(root: &Path, libs_dir: &Path) {
    let proj = root.join("learn-to-compile-parser-c#");

    run(
        Command::new("dotnet")
            .args(["publish", "-c", "Release", "-r", "linux-x64"])
            .current_dir(&proj),
        "C# dotnet publish",
    );

    let publish_dir = proj.join("bin/Release/net8.0/linux-x64/publish");
    let so = publish_dir.join("liblearn-to-compile-c-sharp.so");

    copy_to_libs(&so, libs_dir);

    watch_dir(&proj.join("src"), "cs");
    println!("cargo:rerun-if-changed={}", proj.join("**/*.cs").display());
}

fn build_haskell(root: &Path, libs_dir: &Path) {
    let proj = root.join("learn-to-compile-typechecker-haskell"); // adjust folder name

    // Resolve active GHC's lib dir and rts library name dynamically
    let ghc_libdir = String::from_utf8(
        Command::new("ghc")
            .arg("--print-libdir")
            .output()
            .expect("ghc --print-libdir failed")
            .stdout,
    )
    .expect("non-utf8 ghc libdir")
    .trim()
    .to_owned();

    // Derive the rts lib name from the actual libdir path instead of hard-coding it
    // ghc --print-libdir returns something like:
    //   /home/generic/.ghcup/ghc/9.10.3/lib/ghc-9.10.3/lib
    // We need to find libHSrts-*.so inside the rts subdirectory
    // println!("cargo:warning=GHC libdir: {ghc_libdir}");

    let (rts_name, rts_dir) = find_rts_lib(&ghc_libdir).unwrap_or_else(|| {
        panic!(
            "Could not find HSrts-*.so anywhere under {ghc_libdir}. \
             Run: find {ghc_libdir} -name 'libHSrts*.so' to locate it manually."
        )
    });

    println!("cargo:warning=Using RTS: {rts_name} from {rts_dir}");

    // stack build first so dependencies are present
    run(
        Command::new("stack").arg("build").current_dir(&proj),
        "Haskell stack build",
    );

    let out_so = proj.join("libhaskellTypeChecker.so");

    // // Resolve the GHC lib dir at build time so the path isn't hard-coded
    // let ghc_libdir = String::from_utf8(
    //     Command::new("ghc")
    //         .arg("--print-libdir")
    //         .output()
    //         .expect("ghc --print-libdir failed")
    //         .stdout,
    // )
    // .expect("non-utf8 ghc libdir")
    // .trim()
    // .to_owned();

    run(
        Command::new("stack")
            .args([
                "exec", "--",
                "ghc",
                "-shared", "-dynamic", "-fPIC",
                "-package", "aeson",
                "-package", "bytestring",
                "-outputdir", "/tmp/hs-build",
                "src/ReadAST.hs",
                "-o",
            ])
            .arg(&out_so)
            .current_dir(&proj),
        "Haskell ghc -shared",
    );

    copy_to_libs(&out_so, libs_dir);

    // Link against the rts dir where the .so actually lives
    println!("cargo:rustc-link-search=native={rts_dir}");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{rts_dir}");
    // Also add the parent libdir for other GHC libs
    println!("cargo:rustc-link-search=native={ghc_libdir}");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{ghc_libdir}");
    println!("cargo:rustc-link-lib=dylib={rts_name}");

    watch_dir(&proj.join("src"), "hs");
    println!("cargo:rerun-if-changed={}", proj.join("src/ReadAST.hs").display());
}

// Scans <ghc_libdir>/rts/ for libHSrts-*.so and returns the bare link name
// e.g. "HSrts-1.0.2-ghc9.10.3"
fn find_rts_lib(ghc_libdir: &str) -> Option<(String, String)> {
    // GHC versions place the rts dir in different spots, try both:
    // - <libdir>/rts/
    // - <libdir>/x86_64-linux-ghc-<ver>/rts-<ver>/
    let base = Path::new(ghc_libdir);

    let candidate_dirs: Vec<PathBuf> = {
        let mut dirs = vec![base.join("rts")];

        // Scan for the arch-specific subdir (x86_64-linux-ghc-9.10.3)
        if let Ok(entries) = fs::read_dir(base) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = entry.file_name();
                    let s = name.to_string_lossy();
                    // e.g. x86_64-linux-ghc-9.10.3
                    if s.starts_with("x86_64-") || s.starts_with("aarch64-") {
                        // Inside that dir, scan for rts-* subdirs
                        if let Ok(inner) = fs::read_dir(&path) {
                            for inner_entry in inner.flatten() {
                                let inner_name = inner_entry.file_name();
                                if inner_name.to_string_lossy().starts_with("rts") {
                                    dirs.push(inner_entry.path());
                                }
                            }
                        }
                        // Also try the arch dir itself (some layouts put .so directly there)
                        dirs.push(path);
                    }
                }
            }
        }
        dirs
    };

    for rts_dir in &candidate_dirs {
        println!("cargo:warning=Scanning for HSrts in: {}", rts_dir.display());
        let Ok(entries) = fs::read_dir(rts_dir) else { continue };

        for entry in entries.flatten() {
            let name = entry.file_name();
            let s = name.to_string_lossy();
            // Match libHSrts-*.so but skip threaded (_thr) and debug (_debug) variants
            if s.starts_with("libHSrts-")
                && s.ends_with(".so")
                && !s.contains("_thr")
                && !s.contains("_debug")
                && !s.contains("_l")
            {
                let link_name = s
                    .trim_start_matches("lib")
                    .trim_end_matches(".so")
                    .to_owned();
                let dir_str = rts_dir.to_string_lossy().to_owned().to_string();
                return Some((link_name, dir_str));
            }
        }
    }

    None
}

fn build_scala(root: &Path, libs_dir: &Path) {
    let proj = root.join("learn-to-compile-IR-tac-scala");

    run(
        Command::new("sbt").arg("nativeImage").current_dir(&proj),
        "Scala sbt nativeImage",
    );

    let native_image_dir = proj.join("target/native-image");
    let so = native_image_dir.join("learn-about-compilers-scala.so");

    if !so.exists() {
        // Diagnostic fallback — print dir contents so future renames are obvious
        println!("cargo:warning=Scala native-image dir contents:");
        if let Ok(entries) = fs::read_dir(&native_image_dir) {
            for entry in entries.flatten() {
                println!("cargo:warning=  {}", entry.file_name().to_string_lossy());
            }
        }
        panic!("Scala .so not found at {}", so.display());
    }

    // // Print what Graal actually produced so we know the real filename
    // println!("cargo:warning=Scala native-image dir contents:");
    // if let Ok(entries) = fs::read_dir(&native_image_dir) {
    //     for entry in entries.flatten() {
    //         println!("cargo:warning=  {}", entry.path().display());
    //     }
    // }


    // Graal omits the lib prefix — rename to liblearn-about-compilers-scala.so
    // so that rustc's dylib=learn-about-compilers-scala directive resolves correctly
    let so_renamed = native_image_dir.join("liblearn-about-compilers-scala.so");
    fs::copy(&so, &so_renamed)
        .unwrap_or_else(|e| panic!("Failed to rename Scala .so: {e}"));

    copy_to_libs(&so_renamed, libs_dir);

    watch_dir(&proj.join("src"), "scala");
    println!("cargo:rerun-if-changed={}", proj.join("src").display());

}

fn build_go(root: &Path, libs_dir: &Path) {
    let proj = root.join("learn-to-compile-bytecode-go");

    let out_so = proj.join("libtacbytecode.so");

    run(
        Command::new("go")
            .args(["build", "-buildmode=c-shared", "-o"])
            .arg(&out_so)
            .arg(".")
            .current_dir(&proj),
        "Go build",
    );

    copy_to_libs(&out_so, libs_dir);
    // Also copy the generated C header if you need it
    let header = proj.join("libtacbytecode.h");
    if header.exists() {
        copy_to_libs(&header, libs_dir);
    }

    watch_dir(&proj, "go");
    println!("cargo:rerun-if-changed={}", proj.join("*.go").display());
}

fn build_zig(root: &Path, libs_dir: &Path) {
    let proj = root.join("learn-to-compile-mapTacAsm-zig");

    run(
        Command::new("zig")
            .args(["build", "-Doptimize=ReleaseSafe"])
            .current_dir(&proj),
        "Zig build",
    );

    let so = proj.join("zig-out/lib/libtac_codegen.so");
    copy_to_libs(&so, libs_dir);

    watch_dir(&proj.join("src"), "zig");
    println!("cargo:rerun-if-changed={}", proj.join("src/ffi.zig").display());
    println!("cargo:rerun-if-changed={}", proj.join("src/asmMap.zig").display());
}

fn build_cpp(root: &Path, libs_dir: &Path) {
    let proj = root
        .join("learn-to-compile-elfgen-c++")
        .canonicalize()
        .expect("could not resolve C++ project path");

    let sources = ["parseAsm", "byteGeneration", "elf"];
    let mut obj_files: Vec<PathBuf> = Vec::new();

    for src in &sources {
        let cpp_file = proj.join(format!("{src}.cpp"));
        let obj_file = proj.join(format!("{src}.o"));

        run(
            Command::new("g++")
                .args(["-O2", "-fPIC", "-c"])
                .arg(&cpp_file)
                .arg("-o")
                .arg(&obj_file),
            &format!("g++ compile {src}.cpp"),
        );

        obj_files.push(obj_file);
        println!("cargo:rerun-if-changed={}", cpp_file.display());
    }

    // Build a shared lib (.so) so it lands in libs/ like everything else.
    // Keep the static .a as well if you prefer — just swap the block below.
    let out_so = proj.join("libparseAsm.so");

    run(
        Command::new("g++")
            .arg("-shared")
            .arg("-o")
            .arg(&out_so)
            .args(&obj_files)
            .arg("-lstdc++"),
        "g++ link shared libparseAsm",
    );

    copy_to_libs(&out_so, libs_dir);
    watch_dir(&proj, "hpp");
}

// ── main ──────────────────────────────────────────────────────────────────

/*
USAGE FOR BUILD FLAGS 
# Normal build — only rebuilds languages whose sources changed
cargo build

# Rebuild everything regardless of changes
REBUILD=all cargo build

# Skip all native builds
REBUILD=none cargo build

# Rebuild only specific languages
REBUILD=scala cargo build
REBUILD=csharp,go cargo build
REBUILD=c++,zig cargo build

export REBUILD=scala
cargo build   # rebuilds scala
cargo build   # still rebuilds scala

# When done, clear it
unset REBUILD
cargo build   # back to normal change detection

 */


fn main() {
    let root = workspace_root();
    let libs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("libs");
    fs::create_dir_all(&libs_dir).expect("could not create libs/");

    // Tell cargo to re-run build.rs itself if this env var changes
    println!("cargo:rerun-if-env-changed=REBUILD");

    let flags = BuildFlags::from_env();
    println!("cargo:warning=Build flags: {flags:?}");

    if flags.csharp  { build_csharp(&root, &libs_dir); }
    if flags.haskell { build_haskell(&root, &libs_dir); }
    if flags.scala   { build_scala(&root, &libs_dir); }
    if flags.go      { build_go(&root, &libs_dir); }
    if flags.zig     { build_zig(&root, &libs_dir); }
    if flags.cpp     { build_cpp(&root, &libs_dir); }

    // ── single link-search pointing at libs/ ──────────────────────────────
    let libs_str = libs_dir.display().to_string();
    println!("cargo:rustc-link-search=native={libs_str}");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{libs_str}");

    println!("cargo:rustc-link-lib=dylib=learn-to-compile-c-sharp");
    println!("cargo:rustc-link-lib=dylib=haskellTypeChecker");
    println!("cargo:rustc-link-lib=dylib=learn-about-compilers-scala");
    println!("cargo:rustc-link-lib=dylib=tacbytecode");
    println!("cargo:rustc-link-lib=dylib=tac_codegen");
    println!("cargo:rustc-link-lib=dylib=parseAsm");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}



    // let root = workspace_root();

    // // Central libs directory lives inside the rust crate for simplicity,
    // // but you can move it to `root.join("libs")` if you prefer.
    // let libs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("libs");
    // fs::create_dir_all(&libs_dir).expect("could not create libs/");

    // // ── build every language ───────────────────────────────────────────────
    // build_csharp(&root, &libs_dir);
    // build_haskell(&root, &libs_dir);
    // build_scala(&root, &libs_dir);
    // build_go(&root, &libs_dir);
    // build_zig(&root, &libs_dir);
    // build_cpp(&root, &libs_dir);

    // // ── single link-search pointing at libs/ ──────────────────────────────
    // let libs_str = libs_dir.display().to_string();
    // println!("cargo:rustc-link-search=native={libs_str}");
    // // rpath so the final binary finds the .so files at runtime
    // println!("cargo:rustc-link-arg=-Wl,-rpath,{libs_str}");

    // // ── link directives (names only, search path already set above) ────────
    // println!("cargo:rustc-link-lib=dylib=learn-to-compile-c-sharp");
    // println!("cargo:rustc-link-lib=dylib=haskellTypeChecker");
    // println!("cargo:rustc-link-lib=dylib=learn-about-compilers-scala");
    // println!("cargo:rustc-link-lib=dylib=tacbytecode");
    // println!("cargo:rustc-link-lib=dylib=tac_codegen");
    // println!("cargo:rustc-link-lib=dylib=parseAsm");
    // println!("cargo:rustc-link-lib=dylib=stdc++");