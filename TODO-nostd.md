# TODO: nostd Mode for OS Development

This document tracks planned features for using TypePython to write operating systems, bootloaders, and embedded firmware without standard library dependencies.

## Phase 1: Core nostd Infrastructure

### 1. Conditional Compilation Framework
- [ ] Add `#![no_std]` equivalent flag/annotation
- [ ] Create feature flags: `std`, `alloc`, `nostd`
- [ ] Split runtime into `core` and `std` portions
- [ ] Create build system support for nostd targets

### 2. Freestanding Runtime Core
- [ ] Create `runtime_core/` with no libc dependencies
- [ ] Implement basic types without heap allocation:
  - [ ] Fixed-size integers (i8, i16, i32, i64, u8, u16, u32, u64)
  - [ ] Fixed-size arrays
  - [ ] Tuples (stack-allocated)
  - [ ] References/pointers
- [ ] Implement panic handler interface
- [ ] Implement stack unwinding alternative (or disable exceptions)

### 3. Memory Management Abstraction
- [ ] Define allocator trait/interface:
  ```python
  class Allocator:
      def alloc(self, size: int, align: int) -> int: ...
      def dealloc(self, ptr: int, size: int, align: int) -> None: ...
      def realloc(self, ptr: int, old_size: int, new_size: int, align: int) -> int: ...
  ```
- [ ] Allow pluggable allocators at compile time
- [ ] Implement bump allocator (simple, for bootloaders)
- [ ] Implement fixed-pool allocator (for embedded)
- [ ] Create static allocation mode (no dynamic allocation)

### 4. Entry Point Customization
- [ ] Support custom entry points (not `main`)
- [ ] Support `#[entry]` or `@entry` annotation
- [ ] Generate proper ELF sections for OS loaders
- [ ] Support position-independent code (PIC) generation
- [ ] Support custom linker scripts

## Phase 2: Low-Level Primitives

### 5. Inline Assembly Support
- [ ] Add `asm()` built-in function:
  ```python
  def read_msr(msr: int) -> int:
      return asm("rdmsr", input={"ecx": msr}, output="eax:edx")
  ```
- [ ] Support input/output constraints
- [ ] Support clobber lists
- [ ] Support volatile assembly
- [ ] Architecture-specific assembly (x86_64, RISC-V, ARM64)

### 6. Raw Pointer Operations
- [ ] Add pointer types: `ptr[T]`, `mut_ptr[T]`
- [ ] Implement pointer arithmetic
- [ ] Add `read_volatile()` and `write_volatile()` operations
- [ ] Add `memcpy()`, `memset()`, `memmove()` primitives
- [ ] Support raw memory access with explicit unsafety

### 7. MMIO (Memory-Mapped I/O) Support
- [ ] Create MMIO register abstraction:
  ```python
  class MMIORegister[T]:
      address: int
      def read(self) -> T: ...
      def write(self, value: T) -> None: ...
  ```
- [ ] Support volatile read/write semantics
- [ ] Support bitfield access patterns
- [ ] Generate proper memory barriers

### 8. Interrupt Handling
- [ ] Support interrupt handler functions:
  ```python
  @interrupt_handler
  def timer_interrupt(frame: InterruptFrame) -> None:
      ...
  ```
- [ ] Generate proper interrupt handler prologue/epilogue
- [ ] Support interrupt enable/disable primitives
- [ ] Create IDT/IVT setup helpers (x86_64)
- [ ] Create trap vector setup (RISC-V)

## Phase 3: OS-Specific Features

### 9. Bare Metal Targets
- [ ] Add target: `x86_64-unknown-none`
- [ ] Add target: `riscv64-unknown-none`
- [ ] Add target: `aarch64-unknown-none`
- [ ] Configure proper ABI for each target
- [ ] Remove all libc dependencies for these targets

### 10. Custom Panic Handling
- [ ] Allow user-defined panic handler:
  ```python
  @panic_handler
  def on_panic(info: PanicInfo) -> Never:
      # Custom panic behavior
      halt()
  ```
- [ ] Support panic without unwinding
- [ ] Support panic with stack trace (optional)
- [ ] Create abort-on-panic mode

### 11. Atomic Operations
- [ ] Implement atomic types: `AtomicInt`, `AtomicBool`, `AtomicPtr[T]`
- [ ] Support atomic operations: `load`, `store`, `swap`, `compare_exchange`
- [ ] Support memory orderings: `Relaxed`, `Acquire`, `Release`, `AcqRel`, `SeqCst`
- [ ] Generate proper LLVM atomic instructions

### 12. Spinlocks and Synchronization Primitives
- [ ] Implement `SpinLock[T]`
- [ ] Implement `Mutex[T]` (when scheduler available)
- [ ] Implement memory barriers
- [ ] Support critical sections

### 13. Static Initialization
- [ ] Support compile-time constant evaluation
- [ ] Support static variables with initializers:
  ```python
  @static
  KERNEL_VERSION: str = "1.0.0"
  ```
- [ ] Generate proper `.data`, `.rodata`, `.bss` sections
- [ ] Support lazy static initialization (optional)

## Phase 4: OS Library Components

### 14. Formatting Without Allocation
- [ ] Create `fmt::write()` that writes to a buffer
- [ ] Support integer formatting (decimal, hex, binary)
- [ ] Support fixed-point formatting
- [ ] No heap allocation required

### 15. Ring Buffer / Fixed Collections
- [ ] `FixedVec[T, N]` - stack-allocated vector with max capacity
- [ ] `RingBuffer[T, N]` - circular buffer
- [ ] `StaticString[N]` - fixed-capacity string
- [ ] `BitSet[N]` - fixed-size bit set

### 16. Hardware Abstraction Helpers
- [ ] CPU identification and feature detection
- [ ] Port I/O (x86): `inb()`, `outb()`, `inw()`, `outw()`, etc.
- [ ] CSR access (RISC-V): `csrr()`, `csrw()`, etc.
- [ ] System register access (ARM64)

### 17. Debugging Support
- [ ] Serial/UART output for early debugging
- [ ] Support debug info generation (DWARF)
- [ ] Create minimal printf without heap
- [ ] Support GDB stub integration points

## Phase 5: Example OS Components

### 18. Example: Bootloader
- [ ] Create minimal bootloader example
- [ ] Support Multiboot2 header generation
- [ ] Support UEFI stub generation
- [ ] Create page table setup helpers

### 19. Example: Kernel Module
- [ ] Create kernel module example
- [ ] Show interrupt handling
- [ ] Show memory-mapped device access
- [ ] Show basic scheduler implementation

### 20. Example: Embedded Firmware
- [ ] Create bare-metal embedded example
- [ ] Show GPIO access patterns
- [ ] Show timer/PWM usage
- [ ] Show I2C/SPI communication

---

## Implementation Notes

### Memory Layout for nostd

For OS development, TypePython should generate code with explicit control over:

1. **Stack usage**: Know maximum stack depth at compile time
2. **Heap usage**: Optional, pluggable allocator
3. **Static data**: Proper section placement
4. **Code placement**: Support for different memory regions

### Unsafe Blocks

Consider adding explicit unsafe regions:

```python
@unsafe
def write_to_hardware(addr: int, value: int) -> None:
    ptr: mut_ptr[int] = cast(addr)
    ptr.write_volatile(value)
```

### ABI Compatibility

For OS development, support:
- C ABI for interop with assembly and C code
- Custom calling conventions
- Naked functions (no prologue/epilogue)
- Section attributes for linker control
