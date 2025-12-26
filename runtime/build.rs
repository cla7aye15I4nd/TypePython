use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const MUSL_VERSION: &str = "1.2.5";

/// Get the musl prefix for a target, either from environment variable or by building it
///
/// This checks for MUSL_{TARGET}_PREFIX environment variable first (set by Docker/CI).
/// If not found, it downloads and builds musl from source.
///
/// Returns the prefix path containing lib/ and include/ directories
fn get_musl_prefix(out_path: &Path, target: &str) -> PathBuf {
    let env_var = format!("MUSL_{}_PREFIX", target.to_uppercase().replace("-", "_"));

    // Check if pre-built musl is available via environment variable
    if let Ok(prefix) = env::var(&env_var) {
        let prefix_path = PathBuf::from(&prefix);
        let lib_dir = prefix_path.join("lib");

        // Verify required files exist
        let required_files = ["crt1.o", "crti.o", "crtn.o", "libc.a"];
        let all_exist = required_files.iter().all(|f| lib_dir.join(f).exists());

        if all_exist {
            eprintln!(
                "Using pre-built musl {} from {} ({})",
                target, prefix, env_var
            );
            return prefix_path;
        } else {
            eprintln!(
                "Warning: {} is set but required files are missing, will build from source",
                env_var
            );
        }
    }

    // Fall back to downloading and building musl from source
    eprintln!(
        "No pre-built musl {} found, downloading and building from source...",
        target
    );
    download_and_build_musl(out_path, target)
}

/// Download musl source and build it for the specified target
fn download_and_build_musl(out_path: &Path, target: &str) -> PathBuf {
    let musl_src_dir = out_path.join(format!("musl-{}", MUSL_VERSION));
    let install_prefix = out_path.join(format!("musl-{}-install", target));
    let lib_dir = install_prefix.join("lib");

    // Check if already built (smart caching)
    let required_files = ["crt1.o", "crti.o", "crtn.o", "libc.a"];
    let all_exist = required_files.iter().all(|f| lib_dir.join(f).exists());

    if all_exist {
        eprintln!(
            "musl {} already built at {}",
            target,
            install_prefix.display()
        );
        return install_prefix;
    }

    // Download musl source if not already present
    if !musl_src_dir.exists() {
        eprintln!("Downloading musl {}...", MUSL_VERSION);

        let tarball = out_path.join(format!("musl-{}.tar.gz", MUSL_VERSION));
        let url = format!(
            "https://musl.libc.org/releases/musl-{}.tar.gz",
            MUSL_VERSION
        );

        let status = Command::new("wget")
            .args(["-q", "-O"])
            .arg(&tarball)
            .arg(&url)
            .status()
            .expect("Failed to execute wget. Make sure wget is installed.");

        if !status.success() {
            panic!("Failed to download musl from {}", url);
        }

        // Extract tarball
        let status = Command::new("tar")
            .args(["-xzf"])
            .arg(&tarball)
            .current_dir(out_path)
            .status()
            .expect("Failed to execute tar");

        if !status.success() {
            panic!("Failed to extract musl tarball");
        }

        // Clean up tarball
        fs::remove_file(&tarball).ok();
    }

    eprintln!(
        "Building musl {} from source (this may take several minutes)...",
        target
    );

    // Create build directory for this target
    let build_dir = out_path.join(format!("musl-{}-build", target));
    fs::create_dir_all(&build_dir).expect("Failed to create musl build directory");

    // Configure musl
    let config_mak = build_dir.join("config.mak");
    if !config_mak.exists() {
        eprintln!("Configuring musl for {}...", target);

        let configure_script = musl_src_dir.join("configure");
        let mut cmd = Command::new(&configure_script);
        cmd.current_dir(&build_dir)
            .arg(format!("--target={}-linux-musl", target))
            .arg(format!("--prefix={}", install_prefix.display()))
            .arg("--disable-shared")
            .arg("--enable-static");

        // Set CC and AR environment variables for configure script
        if target == "riscv64" {
            // For RISC-V cross-compilation, use clang with target flag
            let clang_cmd = if let Ok(llvm_prefix) = env::var("LLVM_SYS_211_PREFIX") {
                format!("{}/bin/clang --target=riscv64-linux-musl", llvm_prefix)
            } else {
                "clang --target=riscv64-linux-musl".to_string()
            };

            let ar_cmd = if let Ok(llvm_prefix) = env::var("LLVM_SYS_211_PREFIX") {
                format!("{}/bin/llvm-ar", llvm_prefix)
            } else {
                "llvm-ar".to_string()
            };

            let ranlib_cmd = if let Ok(llvm_prefix) = env::var("LLVM_SYS_211_PREFIX") {
                format!("{}/bin/llvm-ranlib", llvm_prefix)
            } else {
                "llvm-ranlib".to_string()
            };

            cmd.env("CC", clang_cmd);
            cmd.env("AR", ar_cmd);
            cmd.env("RANLIB", ranlib_cmd);
            cmd.env("CFLAGS", "-mabi=lp64d -O2");
        } else {
            // For x86_64, use system gcc
            cmd.env("CC", "gcc");
            cmd.env("AR", "ar");
            cmd.env("RANLIB", "ranlib");
        }

        let status = cmd.status().expect("Failed to execute musl configure");
        if !status.success() {
            panic!(
                "musl configure failed for {}. Make sure make and gcc/clang are installed.",
                target
            );
        }
    }

    // Build CRT objects and libc.a
    eprintln!("Building musl {} objects...", target);

    let num_cpus = std::thread::available_parallelism()
        .map(|n| n.get().to_string())
        .unwrap_or_else(|_| "4".to_string());

    let mut cmd = Command::new("make");
    cmd.current_dir(&build_dir).arg(format!("-j{}", num_cpus));

    if target == "riscv64" {
        let clang_cmd = if let Ok(llvm_prefix) = env::var("LLVM_SYS_211_PREFIX") {
            format!("{}/bin/clang --target=riscv64-linux-musl", llvm_prefix)
        } else {
            "clang --target=riscv64-linux-musl".to_string()
        };

        let ar_cmd = if let Ok(llvm_prefix) = env::var("LLVM_SYS_211_PREFIX") {
            format!("{}/bin/llvm-ar", llvm_prefix)
        } else {
            "llvm-ar".to_string()
        };

        let ranlib_cmd = if let Ok(llvm_prefix) = env::var("LLVM_SYS_211_PREFIX") {
            format!("{}/bin/llvm-ranlib", llvm_prefix)
        } else {
            "llvm-ranlib".to_string()
        };

        cmd.env("CC", clang_cmd);
        cmd.env("AR", ar_cmd);
        cmd.env("RANLIB", ranlib_cmd);
        cmd.env("CFLAGS", "-mabi=lp64d -O2");
    } else {
        cmd.env("AR", "ar");
        cmd.env("RANLIB", "ranlib");
    }

    let status = cmd.status().expect("Failed to execute make");
    if !status.success() {
        panic!("musl build failed for {}", target);
    }

    // Install (creates lib directory with crt*.o and libc.a)
    eprintln!("Installing musl {} libraries...", target);
    let status = Command::new("make")
        .current_dir(&build_dir)
        .arg("install")
        .status()
        .expect("Failed to execute make install");

    if !status.success() {
        panic!("musl install failed for {}", target);
    }

    // Verify all required files were created
    for file in &required_files {
        let file_path = lib_dir.join(file);
        if !file_path.exists() {
            panic!(
                "musl build succeeded but {} is missing at {}",
                file,
                file_path.display()
            );
        }
    }

    eprintln!(
        "musl {} build complete at {}",
        target,
        install_prefix.display()
    );
    install_prefix
}

/// Get the musl include paths for a target architecture
fn musl_include_paths(musl_prefix: &Path, _target: &str) -> Vec<PathBuf> {
    let include_dir = musl_prefix.join("include");

    // For pre-built musl, all headers are in include/
    // Architecture-specific headers (bits/) are already included
    vec![include_dir]
}

/// Build runtime for a specific target architecture using musl
fn build_runtime_for_target(
    out_path: &Path,
    manifest_dir: &Path,
    musl_prefix: &Path,
    target: &str,
) -> PathBuf {
    let c_files = [
        "src/list.c",
        "src/builtins.c",
        "src/class.c",
        "src/bytearray.c",
        "src/str.c",
        "src/bytes.c",
        "src/exception.c",
        "src/range.c",
    ];

    let mut bc_files = Vec::new();

    // Get musl include paths
    let include_paths = musl_include_paths(musl_prefix, target);

    // Get clang target flag for cross-compilation
    let clang_target = match target {
        "riscv64" => Some("--target=riscv64-linux-musl"),
        "x86_64" => None,
        _ => panic!("Unsupported target: {}", target),
    };

    // Compile each C file to LLVM bitcode (.bc)
    for c_file in &c_files {
        let bc_file = out_path.join(format!(
            "{}-{}.bc",
            PathBuf::from(c_file).file_stem().unwrap().to_str().unwrap(),
            target
        ));

        let clang_path = if target == "riscv64" {
            if let Ok(llvm_prefix) = env::var("LLVM_SYS_211_PREFIX") {
                format!("{}/bin/clang", llvm_prefix)
            } else {
                "clang".to_string()
            }
        } else {
            "clang".to_string()
        };

        let mut cmd = Command::new(&clang_path);
        cmd.args(["-c", "-emit-llvm", "-O2"]);

        // Use musl headers instead of system headers
        cmd.arg("-nostdinc");
        for include_path in &include_paths {
            cmd.arg(format!("-isystem{}", include_path.display()));
        }

        if let Some(target_flag) = clang_target {
            cmd.arg(target_flag);
        }

        if target == "riscv64" {
            cmd.arg("-mabi=lp64d");
        }

        let c_file_path = manifest_dir.join(c_file);
        cmd.arg(&c_file_path).arg("-o").arg(&bc_file);

        eprintln!("Running: {:?}", cmd);
        let status = cmd.status().expect("Failed to execute clang");

        if !status.success() {
            panic!("Failed to compile {} to bitcode for {}", c_file, target);
        }

        bc_files.push(bc_file);
    }

    // Link all bitcode files into a single runtime-{target}.o file
    let output_file = out_path.join(format!("runtime-{}.o", target));

    let llvm_link_path = if target == "riscv64" {
        if let Ok(llvm_prefix) = env::var("LLVM_SYS_211_PREFIX") {
            format!("{}/bin/llvm-link", llvm_prefix)
        } else {
            "llvm-link".to_string()
        }
    } else {
        "llvm-link".to_string()
    };

    let status = Command::new(&llvm_link_path)
        .args(&bc_files)
        .arg("-o")
        .arg(&output_file)
        .status()
        .expect("Failed to execute llvm-link");

    if !status.success() {
        panic!("Failed to link runtime bitcode files for {}", target);
    }

    output_file
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(&out_dir);

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = PathBuf::from(&manifest_dir);

    // Build/locate musl for both architectures
    let targets = ["x86_64", "riscv64"];

    eprintln!("Setting up musl C runtime for x86_64 and riscv64...");

    for target in &targets {
        // Get musl prefix (either from env var or by building)
        let musl_prefix = get_musl_prefix(&out_path, target);
        let musl_lib_dir = musl_prefix.join("lib");

        // Export environment variable for the compiler to find musl libraries
        let env_var = format!("MUSL_{}_LIB", target.to_uppercase().replace("-", "_"));
        println!("cargo:rustc-env={}={}", env_var, musl_lib_dir.display());
        eprintln!("Set {} = {}", env_var, musl_lib_dir.display());

        // Build C runtime for this architecture
        build_runtime_for_target(&out_path, &manifest_path, &musl_prefix, target);
    }

    // Tell cargo to rerun if C files change
    println!("cargo:rerun-if-changed=src/list.c");
    println!("cargo:rerun-if-changed=src/builtins.c");
    println!("cargo:rerun-if-changed=src/class.c");
    println!("cargo:rerun-if-changed=src/bytearray.c");
    println!("cargo:rerun-if-changed=src/str.c");
    println!("cargo:rerun-if-changed=src/bytes.c");
    println!("cargo:rerun-if-changed=src/runtime.h");
    println!("cargo:rerun-if-changed=src/types.h");
    println!("cargo:rerun-if-changed=src/memory.h");
    println!("cargo:rerun-if-changed=src/io.h");
    println!("cargo:rerun-if-changed=src/str.h");
    println!("cargo:rerun-if-changed=src/bytes.h");
    println!("cargo:rerun-if-changed=src/exception.c");
    println!("cargo:rerun-if-changed=src/exception.h");
    println!("cargo:rerun-if-changed=src/range.c");

    // Rerun if musl environment variables change
    println!("cargo:rerun-if-env-changed=MUSL_X86_64_PREFIX");
    println!("cargo:rerun-if-env-changed=MUSL_RISCV64_PREFIX");
}
