use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let runtime_dir = PathBuf::from(&manifest_dir).join("src/runtime");
    let builtin_c = runtime_dir.join("builtin.c");
    let builtin_ll = runtime_dir.join("builtin.ll");

    // Tell cargo to rerun if the C source changes
    println!("cargo:rerun-if-changed=src/runtime/builtin.c");

    // Also check if output file exists - if not, we need to build
    let needs_build = !builtin_ll.exists() || {
        // Check if C file is newer than LL file
        let c_meta = std::fs::metadata(&builtin_c).expect("builtin.c not found");
        let ll_meta = std::fs::metadata(&builtin_ll);

        ll_meta.is_err() || c_meta.modified().unwrap() > ll_meta.unwrap().modified().unwrap()
    };

    if !needs_build {
        println!("cargo:warning=Runtime is up to date, skipping build");
        return;
    }

    let llvm_prefix =
        env::var("LLVM_SYS_211_PREFIX").unwrap_or_else(|_| "/usr/lib/llvm-21".to_string());

    let clang = format!("{}/bin/clang", llvm_prefix);

    println!("cargo:warning=Building TypePython runtime library...");
    println!("cargo:warning=Using clang: {}", clang);

    let output = Command::new(&clang)
        .args([
            "-S",
            "-emit-llvm",
            "-O2",
            "-o",
            builtin_ll.to_str().unwrap(),
            builtin_c.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute clang");

    if !output.status.success() {
        panic!(
            "Failed to compile runtime:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!(
        "cargo:warning=Runtime built successfully: {}",
        builtin_ll.display()
    );
}
