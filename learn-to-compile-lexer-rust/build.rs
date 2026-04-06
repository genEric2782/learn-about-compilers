fn main() {
    println!("cargo:rustc-link-search=native=../learn-to-compile-parser-c#/bin/Release/net8.0/linux-x64/publish");
    println!("cargo:rustc-link-lib=dylib=learn-to-compile-c-sharp");
}