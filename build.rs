//! Compiles the two C/C++ wrappers (`csrc/`) over the vendored sources
//! (`vendor/`).
//!
//! - `csrc/a5hash_wrapper.c` on every target.
//! - `csrc/meow_wrapper.cpp` only on x86_64: MeowHash 0.5 exists
//!   for x86_64 with AES-NI only. On any other target the extension is built
//!   without it and `speedupy_native.HAS_MEOWHASH` is false.

use std::env;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    for path in [
        "csrc/a5hash_wrapper.c",
        "csrc/meow_wrapper.cpp",
        "vendor/a5hash/a5hash.h",
        "vendor/meow_hash/meow_hash_x64_aesni.h",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }

    cc::Build::new()
        .file("csrc/a5hash_wrapper.c")
        .include("vendor/a5hash")
        .opt_level(3)
        .warnings(false)
        .compile("a5hash_wrapper");

    if target_arch == "x86_64" {
        let mut meow = cc::Build::new();
        meow.cpp(true)
            .file("csrc/meow_wrapper.cpp")
            .include("vendor/meow_hash")
            .opt_level(3)
            .warnings(false);
        // GCC and Clang need the instruction sets enabled; MSVC compiles the
        // AES intrinsics without a flag, so these are skipped there.
        meow.flag_if_supported("-maes")
            .flag_if_supported("-msse4.1");
        meow.compile("meow_wrapper");
        println!("cargo:rustc-cfg=has_meowhash");
    }
    println!("cargo:rustc-check-cfg=cfg(has_meowhash)");
}
