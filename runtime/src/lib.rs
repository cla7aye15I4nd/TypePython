// This is a placeholder for the runtime crate
// The actual runtime is implemented in C (list.c, builtins.c)
// and linked statically via build.rs

/// Path to the musl x86_64 library directory (set at compile time by build.rs)
pub const MUSL_X86_64_LIB: &str = env!("MUSL_X86_64_LIB");

/// Path to the musl riscv64 library directory (set at compile time by build.rs)
pub const MUSL_RISCV64_LIB: &str = env!("MUSL_RISCV64_LIB");

/// Path to the ICU x86_64 library directory (set at compile time by build.rs)
pub const ICU_X86_64_LIB: &str = env!("ICU_X86_64_LIB");

/// Path to the ICU riscv64 library directory (set at compile time by build.rs)
pub const ICU_RISCV64_LIB: &str = env!("ICU_RISCV64_LIB");

/// Path to the ICU x86_64 include directory (set at compile time by build.rs)
pub const ICU_X86_64_INCLUDE: &str = env!("ICU_X86_64_INCLUDE");

/// Path to the ICU riscv64 include directory (set at compile time by build.rs)
pub const ICU_RISCV64_INCLUDE: &str = env!("ICU_RISCV64_INCLUDE");

/// Path to the libc++ x86_64 library directory for static C++ linking (set at compile time by build.rs)
pub const LIBCXX_X86_64_LIB: &str = env!("LIBCXX_X86_64_LIB");

/// Path to the libc++ riscv64 library directory for static C++ linking (set at compile time by build.rs)
pub const LIBCXX_RISCV64_LIB: &str = env!("LIBCXX_RISCV64_LIB");
