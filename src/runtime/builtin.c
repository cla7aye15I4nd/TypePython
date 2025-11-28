#include <stdio.h>
#include <stdint.h>
#include <math.h>

// TypePython Runtime Library
// Builtin functions implemented in C and compiled to LLVM IR
// These implement Python-like print that handles integers, floats, booleans, and strings
// Printing is separated into value printing and separators (space/newline)

void tpy_print_int(int64_t value) {
    printf("%ld", (long)value);
}

void tpy_print_float(double value) {
    // Check if it's a whole number
    if (value == floor(value)) {
        printf("%.1f", value);
    } else {
        printf("%.15g", value);
    }
}

void tpy_print_bool(_Bool value) {
    printf("%s", value ? "True" : "False");
}

void tpy_print_str(const char* value) {
    printf("%s", value);
}

// Print separator functions
void tpy_print_space(void) {
    printf(" ");
}

void tpy_print_newline(void) {
    printf("\n");
}

// Math functions
double tpy_pow(double base, double exponent) {
    return pow(base, exponent);
}

double tpy_floor(double value) {
    return floor(value);
}

// Integer power function (returns int)
int64_t tpy_pow_int(int64_t base, int64_t exponent) {
    if (exponent < 0) {
        return 0; // Integer division result for negative exponents
    }

    int64_t result = 1;
    int64_t b = base;
    int64_t e = exponent;

    while (e > 0) {
        if (e & 1) {
            result *= b;
        }
        b *= b;
        e >>= 1;
    }

    return result;
}
