#include "runtime.h"
#include <stdio.h>
#include <string.h>

// ============================================================================
// Built-in print functions
// ============================================================================

void __pyc___builtin___int___print__(int64_t value) {
    char buffer[21];
    snprintf(buffer, sizeof(buffer), "%ld", value);
    write_stdout(buffer, strlen(buffer));
}

void __pyc___builtin___bool___print__(int8_t value) {
    if (value) {
        write_stdout("True", 4);
    } else {
        write_stdout("False", 5);
    }
}

void __pyc___builtin___float___print__(double value) {
    char buffer[32];
    snprintf(buffer, sizeof(buffer), "%g", value);
    write_stdout(buffer, strlen(buffer));
}

// ============================================================================
// I/O helper functions for compiler use
// ============================================================================

void write_str_impl(const char* str) {
    write_stdout(str, strlen(str));
}

void write_string_impl(String* str) {
    if (str != NULL) {
        write_stdout(str->data, (size_t)str->len);
    }
}

void write_char_impl(char c) {
    write_char(c);
}

void write_newline_impl(void) {
    putchar('\n');
}

void write_space_impl(void) {
    putchar(' ');
}

char* int64_to_str_impl(int64_t value, char* buffer) {
    snprintf(buffer, 21, "%ld", value);
    return buffer;
}
