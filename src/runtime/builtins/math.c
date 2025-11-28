// TypePython Runtime Library - Math Module
// Builtin math functions for numerical operations

#include <stdint.h>
#include <math.h>

// ============================================================================
// Internal operator support functions (used by binary/unary operators)
// ============================================================================

// Float power function (for ** operator)
double pow_float(double base, double exponent) {
    return pow(base, exponent);
}

// Integer power function (for ** operator)
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

// Float floor function (internal)
double floor_float(double value) {
    return floor(value);
}

// Python-style floor division for integers (for // operator)
int64_t floordiv_int(int64_t a, int64_t b) {
    int64_t q = a / b;
    int64_t r = a % b;
    if ((r != 0) && ((r < 0) != (b < 0))) {
        q -= 1;
    }
    return q;
}

// Python-style modulo for integers (for % operator)
int64_t mod_int(int64_t a, int64_t b) {
    int64_t r = a % b;
    if ((r != 0) && ((r < 0) != (b < 0))) {
        r += b;
    }
    return r;
}

// Python-style modulo for floats (for % operator)
double mod_float(double a, double b) {
    double r = fmod(a, b);
    if ((r != 0.0) && ((r < 0.0) != (b < 0.0))) {
        r += b;
    }
    return r;
}

// ============================================================================
// Python built-in functions: abs()
// ============================================================================

// abs() for integers
int64_t abs_int(int64_t value) {
    return value < 0 ? -value : value;
}

// abs() for floats
double abs_float(double value) {
    return fabs(value);
}

// ============================================================================
// Python built-in functions: round()
// ============================================================================

// round() for floats - rounds to nearest integer using banker's rounding
int64_t round_float(double value) {
    return (int64_t)round(value);
}

// round() for floats with ndigits - returns float
double round_float_ndigits(double value, int64_t ndigits) {
    double multiplier = pow(10.0, (double)ndigits);
    return round(value * multiplier) / multiplier;
}

// round() for integers - returns the integer unchanged
int64_t round_int(int64_t value) {
    return value;
}

// ============================================================================
// Python built-in functions: min() and max()
// ============================================================================

// min() for two integers
int64_t min_int(int64_t a, int64_t b) {
    return a < b ? a : b;
}

// min() for two floats
double min_float(double a, double b) {
    return a < b ? a : b;
}

// max() for two integers
int64_t max_int(int64_t a, int64_t b) {
    return a > b ? a : b;
}

// max() for two floats
double max_float(double a, double b) {
    return a > b ? a : b;
}

// ============================================================================
// Python built-in functions: divmod()
// Returns quotient and remainder as separate values (caller handles tuple)
// ============================================================================

// divmod() quotient for integers (Python-style floor division)
int64_t divmod_int_quot(int64_t a, int64_t b) {
    return floordiv_int(a, b);
}

// divmod() remainder for integers (Python-style modulo)
int64_t divmod_int_rem(int64_t a, int64_t b) {
    return mod_int(a, b);
}

// divmod() quotient for floats
double divmod_float_quot(double a, double b) {
    return floor(a / b);
}

// divmod() remainder for floats (Python-style modulo)
double divmod_float_rem(double a, double b) {
    return mod_float(a, b);
}

// ============================================================================
// Python built-in functions: pow() with optional modulo
// ============================================================================

// pow() for three integers: (base ** exp) % mod
int64_t pow_int_mod(int64_t base, int64_t exponent, int64_t modulo) {
    if (exponent < 0) {
        return 0;  // Negative exponent with modulo not supported for integers
    }

    int64_t result = 1;
    base = mod_int(base, modulo);

    while (exponent > 0) {
        if (exponent & 1) {
            result = mod_int(result * base, modulo);
        }
        exponent >>= 1;
        base = mod_int(base * base, modulo);
    }

    return result;
}
