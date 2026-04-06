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
 
}