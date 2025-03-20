// Copyright (c) 2025, Ibrahim Erturk <me@erturk.me>, ErturkMe
// Licensed under the BSD 3-Clause License.
// See the LICENSE file in the project root for more information.

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("Running build.rs script");

    // Check the target architecture
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    if target_arch == "x86" {
        println!("Building for server and client for x86 target");

        // Enable the server feature
        println!("cargo:rustc-cfg=feature=\"server\"");

        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let lib_dir = manifest_dir
            .join("resource/rbic1-dll")
            .canonicalize()
            .unwrap();
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=dylib=RBIC1");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        let target_dir = PathBuf::from(out_path.clone())
            .join("../../../..")
            .canonicalize()
            .unwrap();
        let profile = env::var("PROFILE").unwrap();
        let target_executable_dir = target_dir.join(profile).join("deps");
        let dll_path = PathBuf::from("resource/rbic1-dll/RBIC1.dll");

        // Copy the DLL to the target executable directory
        fs::copy(&dll_path, target_executable_dir.join("RBIC1.dll"))
            .expect("Failed to copy DLL file to executable directory");
    } else {
        println!("Building client only for x86_64 target");
    }
}
