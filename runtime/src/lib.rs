// This is a placeholder for the runtime crate
// The actual runtime is implemented in C (list.c, builtins.c)
// and linked statically via build.rs

/// Path to the musl x86_64 library directory (set at compile time by build.rs)
pub const MUSL_X86_64_LIB: &str = env!("MUSL_X86_64_LIB");

/// Path to the musl riscv64 library directory (set at compile time by build.rs)
pub const MUSL_RISCV64_LIB: &str = env!("MUSL_RISCV64_LIB");
