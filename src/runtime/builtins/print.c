// TypePython Runtime Library - Print Module
// Builtin print functions for outputting values to stdout

#include <stdio.h>
#include <stdint.h>

void print_int(int64_t value) {
    printf("%ld", (long)value);
}

void print_float(double value) {
    // Check if it's a whole number (without using libm's floor)
    // Cast to int64 and back - if equal, it's a whole number
    int64_t as_int = (int64_t)value;
    if ((double)as_int == value && value >= -9007199254740992.0 && value <= 9007199254740992.0) {
        printf("%.1f", value);
    } else {
        // Try minimal precision first, increase until round-trip works
        // This approximates Python's shortest-representation algorithm
        char buf[32];
        double reparsed;

        // Try 15 digits first (covers most cases cleanly)
        snprintf(buf, sizeof(buf), "%.15g", value);
        sscanf(buf, "%lf", &reparsed);
        if (reparsed == value) {
            printf("%s", buf);
            return;
        }

        // Try 16 digits
        snprintf(buf, sizeof(buf), "%.16g", value);
        sscanf(buf, "%lf", &reparsed);
        if (reparsed == value) {
            printf("%s", buf);
            return;
        }

        // Fall back to 17 digits for full precision
        printf("%.17g", value);
    }
}

void print_bool(_Bool value) {
    printf("%s", value ? "True" : "False");
}

void print_bytes(const char* value) {
    // Print bytes in Python-style format: b'...' with escaped special chars
    printf("b'");
    for (const char* p = value; *p != '\0'; p++) {
        unsigned char c = (unsigned char)*p;
        switch (c) {
            case '\n': printf("\\n"); break;
            case '\t': printf("\\t"); break;
            case '\r': printf("\\r"); break;
            case '\\': printf("\\\\"); break;
            case '\'': printf("\\'"); break;
            default:
                if (c >= 32 && c < 127) {
                    putchar(c);
                } else {
                    printf("\\x%02x", c);
                }
                break;
        }
    }
    printf("'");
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
