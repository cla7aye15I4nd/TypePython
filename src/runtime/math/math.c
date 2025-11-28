// TypePython Runtime Library - Math Module
// Builtin math functions for numerical operations

#include <stdint.h>
#include <math.h>

// Float power function
double pow_float(double base, double exponent) {
    return pow(base, exponent);
}

// Integer power function
int64_t pow_int(int64_t base, int64_t exponent) {
    if (exponent < 0) {
        return 0;
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

// Float floor function
double floor_float(double value) {
    return floor(value);
}

// Python-style floor division for integers
int64_t floordiv_int(int64_t a, int64_t b) {
    int64_t q = a / b;
    int64_t r = a % b;
    if ((r != 0) && ((r < 0) != (b < 0))) {
        q -= 1;
    }
    return q;
}

// Python-style modulo for integers
int64_t mod_int(int64_t a, int64_t b) {
    int64_t r = a % b;
    if ((r != 0) && ((r < 0) != (b < 0))) {
        r += b;
    }
    return r;
}

// Python-style modulo for floats
double mod_float(double a, double b) {
    double r = fmod(a, b);
    if ((r != 0.0) && ((r < 0.0) != (b < 0.0))) {
        r += b;
    }
    return r;
}
