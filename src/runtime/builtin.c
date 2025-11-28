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
    // Print bytes in Python-style format: b'...'
    printf("b'%s'", value);
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

// String operations (implemented as C-style null-terminated strings)
// Note: TypePython str type is currently implemented as C-style char* pointers
// until full Python string objects with UTF-8 support are added
#include <string.h>
#include <stdlib.h>

// String concatenation - allocates new C-string
// Implements the + operator for str type (C-style strings)
const char* tpy_strcat(const char* s1, const char* s2) {
    size_t len1 = strlen(s1);
    size_t len2 = strlen(s2);
    char* result = (char*)malloc(len1 + len2 + 1);
    strcpy(result, s1);
    strcat(result, s2);
    return result;
}

// String comparison - returns 1 if equal, 0 if not
// Implements == operator for str type (C-style strings)
int64_t tpy_strcmp(const char* s1, const char* s2) {
    return strcmp(s1, s2) == 0 ? 1 : 0;
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

// Python-style floor division for integers
// Rounds towards negative infinity (unlike C which truncates towards zero)
int64_t tpy_floordiv_int(int64_t a, int64_t b) {
    int64_t q = a / b;
    int64_t r = a % b;
    // If remainder is non-zero and signs differ, adjust quotient
    if ((r != 0) && ((r < 0) != (b < 0))) {
        q -= 1;
    }
    return q;
}

// Python-style modulo for integers
// Result has the same sign as the divisor (unlike C where it matches the dividend)
int64_t tpy_mod_int(int64_t a, int64_t b) {
    int64_t r = a % b;
    // If remainder is non-zero and signs differ, adjust remainder
    if ((r != 0) && ((r < 0) != (b < 0))) {
        r += b;
    }
    return r;
}

// Python-style modulo for floats
double tpy_fmod(double a, double b) {
    double r = fmod(a, b);
    // If remainder is non-zero and signs differ, adjust remainder
    if ((r != 0.0) && ((r < 0.0) != (b < 0.0))) {
        r += b;
    }
    return r;
}
