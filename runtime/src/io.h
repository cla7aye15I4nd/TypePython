#ifndef IO_H
#define IO_H

// ============================================================================
// I/O functions via musl libc
// Provides simple wrappers around stdio for the runtime
// ============================================================================

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "types.h"

// ============================================================================
// Output functions
// ============================================================================

static inline void write_stdout(const char* str, size_t len) {
    fwrite(str, 1, len, stdout);
}

static inline void write_stderr(const char* str, size_t len) {
    fwrite(str, 1, len, stderr);
}

static inline void write_char(char c) {
    putchar(c);
}

// ============================================================================
// Error handling
// ============================================================================

static inline void rt_panic(const char* message) {
    fprintf(stderr, "Error: %s\n", message);
    exit(1);
}

static inline void rt_panic_index(const char* message, int64_t index, int64_t length) {
    fprintf(stderr, "Error: %s: %ld (length: %ld)\n", message, index, length);
    exit(1);
}

#endif // IO_H
