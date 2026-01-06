use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const MUSL_VERSION: &str = "1.2.5";
const ICU_VERSION: &str = "74.2";
const ICU_VERSION_UNDERSCORE: &str = "74_2"; // for download URL

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

/// Get the ICU prefix for a target, either from environment variable or by building it
fn get_icu_prefix(out_path: &Path, target: &str, musl_prefix: &Path) -> PathBuf {
    let env_var = format!("ICU_{}_PREFIX", target.to_uppercase().replace("-", "_"));

    // Check if pre-built ICU is available via environment variable
    if let Ok(prefix) = env::var(&env_var) {
        let prefix_path = PathBuf::from(&prefix);
        let lib_dir = prefix_path.join("lib");

        // Verify required files exist
        let required_files = ["libicuuc.a", "libicudata.a", "libicui18n.a"];
        let all_exist = required_files.iter().all(|f| lib_dir.join(f).exists());

        if all_exist {
            eprintln!(
                "Using pre-built ICU {} from {} ({})",
                target, prefix, env_var
            );
            return prefix_path;
        } else {
            eprintln!(
                "Warning: {} is set but required files are missing, will try system ICU",
                env_var
            );
        }
    }

    // Try to use system ICU static libraries first (most common case)
    // This works because we include glibc_compat.c for compatibility shims
    if target == "x86_64" {
        let system_icu = PathBuf::from("/usr");
        let system_lib = system_icu.join("lib/x86_64-linux-gnu");
        let required_files = ["libicuuc.a", "libicudata.a", "libicui18n.a"];
        let all_exist = required_files.iter().all(|f| system_lib.join(f).exists());

        if all_exist {
            eprintln!("Using system ICU static libraries for x86_64");
            return system_icu;
        }
    }

    // For RISC-V, cross-compiling ICU requires a C++ standard library that works with musl.
    // Since GCC's libstdc++ depends on glibc-specific features and LLVM's libc++ isn't
    // available for RISC-V musl, we skip ICU for RISC-V by default.
    // Users can provide pre-built ICU via ICU_RISCV64_PREFIX environment variable.
    if target == "riscv64" {
        eprintln!("NOTE: ICU for RISC-V requires pre-built libraries due to C++ stdlib issues.");
        eprintln!("RISC-V binaries will use ASCII-only string operations.");
        eprintln!(
            "To enable full Unicode, set ICU_RISCV64_PREFIX to a pre-built ICU installation."
        );
        return out_path.join("icu-riscv64-placeholder");
    }

    // Fall back to downloading and building ICU from source (x86_64 only)
    eprintln!(
        "No pre-built ICU {} found, downloading and building from source...",
        target
    );
    download_and_build_icu(out_path, target, musl_prefix)
}

/// Download ICU source if not already present
fn download_icu_source(out_path: &Path) {
    let icu_src_dir = out_path.join(format!("icu-{}", ICU_VERSION));

    if icu_src_dir.exists() {
        eprintln!("ICU source already downloaded");
        return;
    }

    eprintln!("Downloading ICU {}...", ICU_VERSION);

    let tarball = out_path.join(format!("icu4c-{}-src.tgz", ICU_VERSION_UNDERSCORE));
    let url = format!(
        "https://github.com/unicode-org/icu/releases/download/release-{}/icu4c-{}-src.tgz",
        ICU_VERSION.replace(".", "-"),
        ICU_VERSION_UNDERSCORE
    );

    let status = Command::new("wget")
        .args(["-q", "-O"])
        .arg(&tarball)
        .arg(&url)
        .status()
        .expect("Failed to execute wget. Make sure wget is installed.");

    if !status.success() {
        panic!("Failed to download ICU from {}", url);
    }

    // Extract tarball (creates icu/ directory with source/ subdirectory)
    let status = Command::new("tar")
        .args(["-xzf"])
        .arg(&tarball)
        .current_dir(out_path)
        .status()
        .expect("Failed to execute tar");

    if !status.success() {
        panic!("Failed to extract ICU tarball");
    }

    // Rename icu/ to icu-{VERSION}/
    fs::rename(out_path.join("icu"), &icu_src_dir).expect("Failed to rename ICU directory");

    // Clean up tarball
    fs::remove_file(&tarball).ok();
}

/// Build ICU native tools (required for cross-compilation)
fn build_icu_native_tools(icu_src_dir: &Path, out_path: &Path) -> PathBuf {
    let native_build_dir = out_path.join("icu-native-build");

    // Check if already built
    let icupkg_bin = native_build_dir.join("bin/icupkg");
    if icupkg_bin.exists() {
        eprintln!("ICU native tools already built");
        return native_build_dir;
    }

    eprintln!("Building ICU native tools...");
    fs::create_dir_all(&native_build_dir).expect("Failed to create native build directory");

    let source_dir = icu_src_dir.join("source");
    let configure_script = source_dir.join("configure");

    // Configure for native build - use plain configure instead of runConfigureICU
    let mut cmd = Command::new(&configure_script);
    cmd.arg(format!("--prefix={}", native_build_dir.display()))
        .arg("--disable-tests")
        .arg("--disable-samples")
        .current_dir(&native_build_dir);

    // Remove potentially conflicting environment variables
    cmd.env_remove("TARGET");
    cmd.env_remove("HOST");

    let status = cmd.status().expect("Failed to execute ICU configure");

    if !status.success() {
        panic!("ICU native tools configure failed");
    }

    // Build
    let num_cpus = std::thread::available_parallelism()
        .map(|n| n.get().to_string())
        .unwrap_or_else(|_| "4".to_string());

    let mut make_cmd = Command::new("make");
    make_cmd
        .arg(format!("-j{}", num_cpus))
        .current_dir(&native_build_dir);

    // Remove Cargo's TARGET/HOST variables which confuse ICU's build system
    make_cmd.env_remove("TARGET");
    make_cmd.env_remove("HOST");
    make_cmd.env_remove("MAKEFLAGS");

    let status = make_cmd.status().expect("Failed to execute make");

    if !status.success() {
        panic!("ICU native tools build failed");
    }

    native_build_dir
}

/// Build ICU for a specific target architecture with musl
fn build_icu_for_target(
    icu_src_dir: &Path,
    install_prefix: &Path,
    target: &str,
    musl_prefix: &Path,
    native_build_dir: &Path,
) {
    let build_dir = install_prefix
        .parent()
        .unwrap()
        .join(format!("icu-{}-build", target));
    fs::create_dir_all(&build_dir).expect("Failed to create ICU build directory");

    let source_dir = icu_src_dir.join("source");
    let configure_script = source_dir.join("configure");

    let musl_include = musl_prefix.join("include");
    let musl_lib = musl_prefix.join("lib");

    // Get LLVM paths for libc++
    let llvm_prefix =
        env::var("LLVM_SYS_211_PREFIX").unwrap_or_else(|_| "/usr/lib/llvm-21".to_string());
    let libcxx_include = format!("{}/include/c++/v1", llvm_prefix);
    let libcxx_lib = format!("{}/lib", llvm_prefix);

    // Configure command
    let mut cmd = Command::new(&configure_script);
    cmd.current_dir(&build_dir)
        .arg(format!("--prefix={}", install_prefix.display()))
        .arg("--enable-static")
        .arg("--disable-shared")
        .arg("--disable-dyload")
        .arg("--disable-tests")
        .arg("--disable-samples")
        .arg("--with-data-packaging=static")
        .arg("--enable-rpath=no")
        .arg(format!("--with-cross-build={}", native_build_dir.display()));

    // Target-specific configuration
    if target == "riscv64" {
        cmd.arg("--host=riscv64-linux-musl");

        // Use GCC cross-compiler's libstdc++ for RISC-V
        let gcc_lib = "/usr/lib/gcc-cross/riscv64-linux-gnu/13";
        let gcc_cxx_include = "/usr/riscv64-linux-gnu/include/c++/13";
        let gcc_cxx_include_arch = "/usr/riscv64-linux-gnu/include/c++/13/riscv64-linux-gnu";

        let clang_path = format!("{}/bin/clang", llvm_prefix);
        let clangxx_path = format!("{}/bin/clang++", llvm_prefix);

        let cc = format!("{} --target=riscv64-linux-musl", clang_path);
        let cxx = format!("{} --target=riscv64-linux-musl", clangxx_path);

        // Need to specify paths for both musl CRT and GCC CRT objects
        let crt_flags = format!("-B{} -B{}", gcc_lib, musl_lib.display());

        cmd.env("CC", format!("{} {}", cc, crt_flags))
            .env("CXX", format!("{} {}", cxx, crt_flags))
            .env("AR", format!("{}/bin/llvm-ar", llvm_prefix))
            .env("RANLIB", format!("{}/bin/llvm-ranlib", llvm_prefix))
            .env(
                "CFLAGS",
                format!(
                    "-nostdinc -isystem{} -O2 -fPIC -mabi=lp64d",
                    musl_include.display()
                ),
            )
            .env(
                "CXXFLAGS",
                format!(
                    "-nostdinc++ -nostdinc -isystem{} -isystem{} -isystem{} -O2 -fPIC -mabi=lp64d -D__GLIBC_PREREQ\\(x,y\\)=0",
                    gcc_cxx_include, gcc_cxx_include_arch, musl_include.display()
                ),
            )
            .env(
                "LDFLAGS",
                format!(
                    "-fuse-ld=lld -static -L{} -L{}",
                    gcc_lib, musl_lib.display()
                ),
            );
    } else {
        // x86_64
        cmd.env("CC", "clang")
            .env("CXX", "clang++")
            .env("AR", "llvm-ar")
            .env("RANLIB", "llvm-ranlib")
            .env(
                "CFLAGS",
                format!("-nostdinc -isystem{} -O2 -fPIC", musl_include.display()),
            )
            .env(
                "CXXFLAGS",
                format!(
                    "-nostdinc++ -nostdinc -isystem{} -isystem{} -O2 -fPIC -stdlib=libc++",
                    libcxx_include,
                    musl_include.display()
                ),
            )
            .env(
                "LDFLAGS",
                format!(
                    "-static -nostdlib++ -L{} -L{} -lc++ -lc++abi -lunwind",
                    libcxx_lib,
                    musl_lib.display()
                ),
            );
    }

    eprintln!("Configuring ICU for {}...", target);
    let status = cmd.status().expect("ICU configure failed");
    if !status.success() {
        panic!("ICU configure failed for {}", target);
    }

    // Build
    eprintln!(
        "Building ICU for {} (this may take several minutes)...",
        target
    );
    let num_cpus = std::thread::available_parallelism()
        .map(|n| n.get().to_string())
        .unwrap_or_else(|_| "4".to_string());

    let mut make_cmd = Command::new("make");
    make_cmd
        .arg(format!("-j{}", num_cpus))
        .current_dir(&build_dir);

    // Remove Cargo's TARGET/HOST variables which confuse ICU's build system
    make_cmd.env_remove("TARGET");
    make_cmd.env_remove("HOST");
    make_cmd.env_remove("MAKEFLAGS");

    let status = make_cmd.status().expect("Failed to execute make");

    if !status.success() {
        panic!("ICU build failed for {}", target);
    }

    // Install
    eprintln!("Installing ICU {} libraries...", target);
    let mut install_cmd = Command::new("make");
    install_cmd.arg("install").current_dir(&build_dir);

    // Remove Cargo's TARGET/HOST variables
    install_cmd.env_remove("TARGET");
    install_cmd.env_remove("HOST");
    install_cmd.env_remove("MAKEFLAGS");

    let status = install_cmd
        .status()
        .expect("Failed to execute make install");

    if !status.success() {
        panic!("ICU install failed for {}", target);
    }
}

/// Download and build ICU for the specified target
fn download_and_build_icu(out_path: &Path, target: &str, musl_prefix: &Path) -> PathBuf {
    let icu_src_dir = out_path.join(format!("icu-{}", ICU_VERSION));
    let install_prefix = out_path.join(format!("icu-{}-install", target));
    let lib_dir = install_prefix.join("lib");

    // Check if already built (smart caching)
    let required_files = ["libicuuc.a", "libicudata.a", "libicui18n.a"];
    let all_exist = required_files.iter().all(|f| lib_dir.join(f).exists());

    if all_exist {
        eprintln!(
            "ICU {} already built at {}",
            target,
            install_prefix.display()
        );
        return install_prefix;
    }

    // Download ICU source if not present
    download_icu_source(out_path);

    // Build native tools (shared across targets)
    let native_build_dir = build_icu_native_tools(&icu_src_dir, out_path);

    // Build for target
    build_icu_for_target(
        &icu_src_dir,
        &install_prefix,
        target,
        musl_prefix,
        &native_build_dir,
    );

    // Verify all required files were created
    for file in &required_files {
        let file_path = lib_dir.join(file);
        if !file_path.exists() {
            panic!(
                "ICU build succeeded but {} is missing at {}",
                file,
                file_path.display()
            );
        }
    }

    eprintln!(
        "ICU {} build complete at {}",
        target,
        install_prefix.display()
    );
    install_prefix
}

/// Build runtime for a specific target architecture using musl and ICU
fn build_runtime_for_target(
    out_path: &Path,
    manifest_dir: &Path,
    musl_prefix: &Path,
    icu_prefix: &Path,
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
        "src/glibc_compat.c", // Compatibility shims for glibc functions (needed for system ICU)
    ];

    let mut bc_files = Vec::new();

    // Get musl include paths
    let mut include_paths = musl_include_paths(musl_prefix, target);

    // Add ICU include path (only if ICU is available)
    let icu_available = !icu_prefix.to_string_lossy().contains("placeholder");
    if icu_available {
        let icu_include = icu_prefix.join("include");
        include_paths.push(icu_include);
    }

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

        // Define NO_ICU if ICU is not available
        if !icu_available {
            cmd.arg("-DNO_ICU=1");
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

    // Build/locate musl and ICU for both architectures
    let targets = ["x86_64", "riscv64"];

    eprintln!("Setting up musl C runtime and ICU for x86_64 and riscv64...");

    for target in &targets {
        // Get musl prefix (either from env var or by building)
        let musl_prefix = get_musl_prefix(&out_path, target);
        let musl_lib_dir = musl_prefix.join("lib");

        // Export environment variable for the compiler to find musl libraries
        let env_var = format!("MUSL_{}_LIB", target.to_uppercase().replace("-", "_"));
        println!("cargo:rustc-env={}={}", env_var, musl_lib_dir.display());
        eprintln!("Set {} = {}", env_var, musl_lib_dir.display());

        // Get ICU prefix (either from env var, system, or by building from source)
        let icu_prefix = get_icu_prefix(&out_path, target, &musl_prefix);

        // Determine ICU lib and include directories based on prefix
        let (icu_lib_dir, icu_include_dir) = if icu_prefix.to_string_lossy().contains("placeholder")
        {
            // Placeholder path for targets without ICU (e.g., RISC-V)
            (icu_prefix.clone(), icu_prefix.clone())
        } else if icu_prefix.to_str() == Some("/usr") {
            // System ICU has arch-specific lib directory
            (
                icu_prefix.join("lib/x86_64-linux-gnu"),
                icu_prefix.join("include"),
            )
        } else {
            // Built from source ICU has lib and include in prefix
            (icu_prefix.join("lib"), icu_prefix.join("include"))
        };

        // Export environment variables for the compiler to find ICU libraries
        let icu_lib_var = format!("ICU_{}_LIB", target.to_uppercase().replace("-", "_"));
        let icu_include_var = format!("ICU_{}_INCLUDE", target.to_uppercase().replace("-", "_"));
        println!("cargo:rustc-env={}={}", icu_lib_var, icu_lib_dir.display());
        println!(
            "cargo:rustc-env={}={}",
            icu_include_var,
            icu_include_dir.display()
        );
        eprintln!("Set {} = {}", icu_lib_var, icu_lib_dir.display());
        eprintln!("Set {} = {}", icu_include_var, icu_include_dir.display());

        // Export libc++ library path for static C++ linking (same for all targets)
        let llvm_prefix =
            env::var("LLVM_SYS_211_PREFIX").unwrap_or_else(|_| "/usr/lib/llvm-21".to_string());
        let libcxx_lib = format!("{}/lib", llvm_prefix);
        let libcxx_var = format!("LIBCXX_{}_LIB", target.to_uppercase().replace("-", "_"));
        println!("cargo:rustc-env={}={}", libcxx_var, libcxx_lib);
        eprintln!("Set {} = {}", libcxx_var, libcxx_lib);

        // Build C runtime for this architecture
        build_runtime_for_target(&out_path, &manifest_path, &musl_prefix, &icu_prefix, target);
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

    // Rerun if ICU environment variables change
    println!("cargo:rerun-if-env-changed=ICU_X86_64_PREFIX");
    println!("cargo:rerun-if-env-changed=ICU_RISCV64_PREFIX");
}
