use std::env;
use std::path::PathBuf;

fn main() {
    // Build C examples
    cc::Build::new()
        .file("examples/c_client.c")
        .include("include")
        .compile("c_client");

    // Tell cargo to invalidate the built crate whenever the C files change
    println!("cargo:rerun-if-changed=examples/c_client.c");
    println!("cargo:rerun-if-changed=include/ffi_leetcode.h");

    // Link the library
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search=native={}", out_path.display());
}
