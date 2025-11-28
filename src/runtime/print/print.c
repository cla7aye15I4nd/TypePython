// TypePython Runtime Library - Print Module
// Builtin print functions for outputting values to stdout

#include <stdio.h>
#include <stdint.h>
#include <math.h>

void print_int(int64_t value) {
    printf("%ld", (long)value);
}

void print_float(double value) {
    // Check if it's a whole number
    if (value == floor(value)) {
        printf("%.1f", value);
    } else {
        printf("%.15g", value);
    }
}

void print_bool(_Bool value) {
    printf("%s", value ? "True" : "False");
}

void print_str(const char* value) {
    // Print bytes in Python-style format: b'...'
    printf("b'%s'", value);
}

void print_space(void) {
    printf(" ");
}

void print_newline(void) {
    printf("\n");
}

void print_none(void) {
    printf("None");
}
