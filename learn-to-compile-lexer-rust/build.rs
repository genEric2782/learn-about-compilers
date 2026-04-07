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
    // Tell Cargo where the compiled Scala native library lives
    // println!("cargo:rustc-link-search=native=../scala-ast/target/native-image"); // TODO
    // println!("cargo:rustc-link-lib=dylib=ast-processor");
    // // GraalVM isolate runtime
    // println!("cargo:rustc-link-lib=dylib=graal_isolate"); // TODO
 
}