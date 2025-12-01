// TypePython Runtime Library - Str Module
// Builtin string/str operations using SDS (Simple Dynamic Strings)
//
// SDS implementation based on Redis SDS library
// Copyright (c) 2006-2015, Salvatore Sanfilippo <antirez at gmail dot com>
// BSD 3-Clause License

#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <ctype.h>
#include <limits.h>
#include <stdarg.h>

#include "../sds.h"

// Additional SDS helpers not in shared sds.h

static void sdsrange(sds s, ssize_t start, ssize_t end) {
    size_t newlen, len = sdslen(s);
    if (len == 0) return;
    if (start < 0) { start = len+start; if (start < 0) start = 0; }
    if (end < 0) { end = len+end; if (end < 0) end = 0; }
    newlen = (start > end) ? 0 : (end-start)+1;
    if (newlen != 0) {
        if (start >= (ssize_t)len) newlen = 0;
        else if (end >= (ssize_t)len) { end = len-1; newlen = (end-start)+1; }
    }
    if (start && newlen) memmove(s, s+start, newlen);
    s[newlen] = 0;
    sdssetlen(s, newlen);
}

static void sdstolower(sds s) {
    size_t len = sdslen(s);
    for (size_t j = 0; j < len; j++) s[j] = tolower(s[j]);
}

static void sdstoupper(sds s) {
    size_t len = sdslen(s);
    for (size_t j = 0; j < len; j++) s[j] = toupper(s[j]);
}

static void sdsclear(sds s) {
    sdssetlen(s, 0);
    s[0] = '\0';
}

// ============================================================================
// TypePython Str API (exported functions)
// ============================================================================

// Note: Input str can be either raw C strings (from literals) or SDS strings.
// For raw C strings we use strlen() to get length.
// Output is always a proper SDS string.

// Helper to get length - works with both raw C strings and SDS
// For raw C strings (from literals), we use strlen
// For SDS strings, the length is stored in the header
static inline size_t str_getlen(const char* s) {
    if (s == NULL) return 0;
    // Just use strlen - it works for both raw C strings and SDS
    // (SDS strings are null-terminated too)
    return strlen(s);
}

// Create new str from C string literal
sds str_new(const char* s) {
    return sdsnew(s);
}

// Create new str with specified length
sds str_newlen(const char* s, int64_t len) {
    return sdsnewlen(s, (size_t)len);
}

// Create empty str
sds str_empty(void) {
    return sdsempty();
}

// Free str
void str_free(sds s) {
    sdsfree(s);
}

// Get length of str (works with raw C strings and SDS)
int64_t str_len(const char* s) {
    if (s == NULL) return 0;
    return (int64_t)strlen(s);
}

// Duplicate str
sds str_dup(const char* s) {
    return sdsnew(s);
}

// Concatenate two str - returns new sds
// Works with raw C strings or SDS strings as input
sds strcat_str(const char* s1, const char* s2) {
    size_t len1 = strlen(s1);
    size_t len2 = strlen(s2);
    sds result = sdsnewlen(NULL, len1 + len2);
    if (result == NULL) return NULL;
    memcpy(result, s1, len1);
    memcpy(result + len1, s2, len2);
    result[len1 + len2] = '\0';
    return result;
}

// Compare two str - returns 1 if equal, 0 if not
// Works with raw C strings or SDS strings
int64_t strcmp_str(const char* s1, const char* s2) {
    return strcmp(s1, s2) == 0 ? 1 : 0;
}

// Compare two str - returns -1, 0, or 1
int64_t str_cmp(const char* s1, const char* s2) {
    int cmp = strcmp(s1, s2);
    if (cmp < 0) return -1;
    if (cmp > 0) return 1;
    return 0;
}

// Less than comparison
int64_t str_lt(const char* s1, const char* s2) {
    return strcmp(s1, s2) < 0 ? 1 : 0;
}

// Less than or equal comparison
int64_t str_le(const char* s1, const char* s2) {
    return strcmp(s1, s2) <= 0 ? 1 : 0;
}

// Greater than comparison
int64_t str_gt(const char* s1, const char* s2) {
    return strcmp(s1, s2) > 0 ? 1 : 0;
}

// Greater than or equal comparison
int64_t str_ge(const char* s1, const char* s2) {
    return strcmp(s1, s2) >= 0 ? 1 : 0;
}

// Repeat str n times
sds strrepeat_str(const char* s, int64_t n) {
    if (n <= 0) return sdsempty();

    size_t len = strlen(s);
    size_t total_len = len * n;
    sds result = sdsnewlen(NULL, total_len);
    if (result == NULL) return NULL;

    for (int64_t i = 0; i < n; i++) {
        memcpy(result + (i * len), s, len);
    }
    result[total_len] = '\0';
    return result;
}

// Get byte at index (returns -1 if out of bounds)
int64_t str_getitem(const char* s, int64_t index) {
    size_t len = strlen(s);
    if (index < 0) index = len + index;
    if (index < 0 || (size_t)index >= len) return -1;
    return (unsigned char)s[index];
}

// Get slice of str (start:end)
sds str_slice(const char* s, int64_t start, int64_t end) {
    size_t len = strlen(s);

    // Handle negative indices
    if (start < 0) start = len + start;
    if (end < 0) end = len + end;

    // Clamp to bounds
    if (start < 0) start = 0;
    if (end < 0) end = 0;
    if ((size_t)start > len) start = len;
    if ((size_t)end > len) end = len;

    // Handle empty slice
    if (start >= end) return sdsempty();

    return sdsnewlen(s + start, end - start);
}

// Get slice of str with step (start:end:step)
// INT64_MAX is used as sentinel for "use default value"
sds str_slice_step(const char* s, int64_t start, int64_t end, int64_t step) {
    if (step == 0) return sdsempty();  // step of 0 is invalid

    size_t len = strlen(s);
    int64_t slen = (int64_t)len;

    // Handle default values based on step direction
    if (start == INT64_MAX) {
        start = (step > 0) ? 0 : slen - 1;
    }
    if (end == INT64_MAX) {
        end = (step > 0) ? slen : -slen - 1;
    }

    if (step > 0) {
        // Forward slice
        if (start < 0) start = slen + start;
        if (end < 0) end = slen + end;
        if (start < 0) start = 0;
        if (end < 0) end = 0;
        if (start > slen) start = slen;
        if (end > slen) end = slen;
        if (start >= end) return sdsempty();

        // Calculate result length
        size_t result_len = (end - start + step - 1) / step;
        sds result = sdsnewlen(NULL, result_len);
        if (result == NULL) return NULL;

        size_t j = 0;
        for (int64_t i = start; i < end && j < result_len; i += step) {
            result[j++] = s[i];
        }
        result[j] = '\0';
        sdssetlen(result, j);
        return result;
    } else {
        // Negative step (reverse slice)
        if (start < 0) start = slen + start;
        if (end < 0) end = slen + end;

        // Clamp start to valid range
        if (start < 0) return sdsempty();
        if (start >= slen) start = slen - 1;

        // end can be -1 to include index 0 (when coming from default -slen-1)
        // After adjustment, end < 0 means "go all the way to before index 0"
        if (end < -1) end = -1;
        if (end >= slen) end = slen - 1;

        if (start <= end) return sdsempty();

        // Calculate result length
        size_t result_len = (start - end + (-step) - 1) / (-step);
        sds result = sdsnewlen(NULL, result_len);
        if (result == NULL) return NULL;

        size_t j = 0;
        for (int64_t i = start; i > end && j < result_len; i += step) {
            result[j++] = s[i];
        }
        result[j] = '\0';
        sdssetlen(result, j);
        return result;
    }
}

// Check if str contains substring
int64_t str_contains(const char* haystack, const char* needle) {
    if (strlen(needle) == 0) return 1;
    return strstr(haystack, needle) != NULL ? 1 : 0;
}

// Find first occurrence of substring (returns -1 if not found)
int64_t str_find(const char* haystack, const char* needle) {
    if (strlen(needle) == 0) return 0;
    char *pos = strstr(haystack, needle);
    if (pos == NULL) return -1;
    return pos - haystack;
}

// Check if str starts with prefix
int64_t str_startswith(const char* s, const char* prefix) {
    size_t slen = strlen(s);
    size_t plen = strlen(prefix);
    if (plen > slen) return 0;
    return memcmp(s, prefix, plen) == 0 ? 1 : 0;
}

// Check if str ends with suffix
int64_t str_endswith(const char* s, const char* suffix) {
    size_t slen = strlen(s);
    size_t suflen = strlen(suffix);
    if (suflen > slen) return 0;
    return memcmp(s + slen - suflen, suffix, suflen) == 0 ? 1 : 0;
}

// Convert to uppercase (returns new str)
sds str_upper(const char* s) {
    sds result = sdsnew(s);
    if (result == NULL) return NULL;
    sdstoupper(result);
    return result;
}

// Convert to lowercase (returns new str)
sds str_lower(const char* s) {
    sds result = sdsnew(s);
    if (result == NULL) return NULL;
    sdstolower(result);
    return result;
}

// Capitalize: first character uppercase, rest lowercase (returns new str)
sds str_capitalize(const char* s) {
    sds result = sdsnew(s);
    if (result == NULL) return NULL;
    size_t len = sdslen(result);
    if (len > 0) {
        result[0] = toupper((unsigned char)result[0]);
        for (size_t i = 1; i < len; i++) {
            result[i] = tolower((unsigned char)result[i]);
        }
    }
    return result;
}

// Title case: uppercase after whitespace, lowercase elsewhere (returns new str)
sds str_title(const char* s) {
    sds result = sdsnew(s);
    if (result == NULL) return NULL;
    size_t len = sdslen(result);
    int in_word = 0;
    for (size_t i = 0; i < len; i++) {
        if (isspace((unsigned char)result[i])) {
            in_word = 0;
        } else if (!in_word) {
            result[i] = toupper((unsigned char)result[i]);
            in_word = 1;
        } else {
            result[i] = tolower((unsigned char)result[i]);
        }
    }
    return result;
}

// Swap case: uppercase to lowercase and vice versa (returns new str)
sds str_swapcase(const char* s) {
    sds result = sdsnew(s);
    if (result == NULL) return NULL;
    size_t len = sdslen(result);
    for (size_t i = 0; i < len; i++) {
        unsigned char c = result[i];
        if (isupper(c)) {
            result[i] = tolower(c);
        } else if (islower(c)) {
            result[i] = toupper(c);
        }
    }
    return result;
}

// Strip whitespace from both ends (returns new str)
sds str_strip(const char* s) {
    sds result = sdsnew(s);
    if (result == NULL) return NULL;
    sdstrim(result, " \t\n\r\f\v");
    return result;
}

// Strip whitespace from left (returns new str)
sds str_lstrip(const char* s) {
    size_t len = strlen(s);
    const char *start = s;
    while (start < s + len && isspace(*start)) start++;
    return sdsnewlen(start, s + len - start);
}

// Strip whitespace from right (returns new str)
sds str_rstrip(const char* s) {
    size_t len = strlen(s);
    const char *end = s + len - 1;
    while (end >= s && isspace(*end)) end--;
    return sdsnewlen(s, end - s + 1);
}

// Replace all occurrences of old with new (returns new str)
sds str_replace(const char* s, const char* old, const char* new_str) {
    size_t slen = strlen(s);
    size_t oldlen = strlen(old);
    size_t newlen = strlen(new_str);

    if (oldlen == 0) return sdsnew(s);

    // Count occurrences
    int count = 0;
    const char *pos = s;
    while ((pos = strstr(pos, old)) != NULL) {
        count++;
        pos += oldlen;
    }

    if (count == 0) return sdsnew(s);

    // Calculate new length and allocate
    size_t result_len = slen + count * (newlen - oldlen);
    sds result = sdsnewlen(NULL, result_len);
    if (result == NULL) return NULL;

    // Build result
    char *dst = result;
    const char *src = s;
    while ((pos = strstr(src, old)) != NULL) {
        size_t before_len = pos - src;
        memcpy(dst, src, before_len);
        dst += before_len;
        memcpy(dst, new_str, newlen);
        dst += newlen;
        src = pos + oldlen;
    }
    // Copy remaining
    strcpy(dst, src);

    return result;
}

// Count occurrences of substring
int64_t str_count(const char* s, const char* sub) {
    size_t sublen = strlen(sub);
    if (sublen == 0) return strlen(s) + 1;

    int64_t count = 0;
    const char *pos = s;
    while ((pos = strstr(pos, sub)) != NULL) {
        count++;
        pos += sublen;
    }
    return count;
}

// Join array of str with separator
sds str_join(const char* sep, char **parts, int64_t count) {
    if (count <= 0) return sdsempty();
    if (count == 1) return sdsnew(parts[0]);

    // Calculate total length
    size_t seplen = strlen(sep);
    size_t total = 0;
    for (int64_t i = 0; i < count; i++) {
        total += strlen(parts[i]);
    }
    total += seplen * (count - 1);

    sds result = sdsnewlen(NULL, total);
    if (result == NULL) return NULL;

    char *dst = result;
    for (int64_t i = 0; i < count; i++) {
        if (i > 0) {
            memcpy(dst, sep, seplen);
            dst += seplen;
        }
        size_t partlen = strlen(parts[i]);
        memcpy(dst, parts[i], partlen);
        dst += partlen;
    }
    result[total] = '\0';

    return result;
}

// Check if all characters are alphanumeric
int64_t str_isalnum(const char* s) {
    size_t len = strlen(s);
    if (len == 0) return 0;
    for (size_t i = 0; i < len; i++) {
        if (!isalnum((unsigned char)s[i])) return 0;
    }
    return 1;
}

// Check if all characters are alphabetic
int64_t str_isalpha(const char* s) {
    size_t len = strlen(s);
    if (len == 0) return 0;
    for (size_t i = 0; i < len; i++) {
        if (!isalpha((unsigned char)s[i])) return 0;
    }
    return 1;
}

// Check if all characters are digits
int64_t str_isdigit(const char* s) {
    size_t len = strlen(s);
    if (len == 0) return 0;
    for (size_t i = 0; i < len; i++) {
        if (!isdigit((unsigned char)s[i])) return 0;
    }
    return 1;
}

// Check if all characters are whitespace
int64_t str_isspace(const char* s) {
    size_t len = strlen(s);
    if (len == 0) return 0;
    for (size_t i = 0; i < len; i++) {
        if (!isspace((unsigned char)s[i])) return 0;
    }
    return 1;
}

// Check if all characters are lowercase
int64_t str_islower(const char* s) {
    size_t len = strlen(s);
    int has_cased = 0;
    for (size_t i = 0; i < len; i++) {
        if (isupper((unsigned char)s[i])) return 0;
        if (islower((unsigned char)s[i])) has_cased = 1;
    }
    return has_cased ? 1 : 0;
}

// Check if all characters are uppercase
int64_t str_isupper(const char* s) {
    size_t len = strlen(s);
    int has_cased = 0;
    for (size_t i = 0; i < len; i++) {
        if (islower((unsigned char)s[i])) return 0;
        if (isupper((unsigned char)s[i])) has_cased = 1;
    }
    return has_cased ? 1 : 0;
}

// Reverse str (returns new str)
sds str_reverse(const char* s) {
    size_t len = strlen(s);
    sds result = sdsnewlen(NULL, len);
    if (result == NULL) return NULL;

    for (size_t i = 0; i < len; i++) {
        result[i] = s[len - 1 - i];
    }
    result[len] = '\0';
    return result;
}

// Center str in field of given width
sds str_center(const char* s, int64_t width) {
    size_t len = strlen(s);
    if ((size_t)width <= len) return sdsnew(s);

    size_t total_pad = width - len;
    size_t left_pad = total_pad / 2;
    size_t right_pad = total_pad - left_pad;

    sds result = sdsnewlen(NULL, width);
    if (result == NULL) return NULL;

    memset(result, ' ', left_pad);
    memcpy(result + left_pad, s, len);
    memset(result + left_pad + len, ' ', right_pad);
    result[width] = '\0';

    return result;
}

// Left-justify str in field of given width
sds str_ljust(const char* s, int64_t width) {
    size_t len = strlen(s);
    if ((size_t)width <= len) return sdsnew(s);

    sds result = sdsnewlen(NULL, width);
    if (result == NULL) return NULL;

    memcpy(result, s, len);
    memset(result + len, ' ', width - len);
    result[width] = '\0';

    return result;
}

// Right-justify str in field of given width
sds str_rjust(const char* s, int64_t width) {
    size_t len = strlen(s);
    if ((size_t)width <= len) return sdsnew(s);

    sds result = sdsnewlen(NULL, width);
    if (result == NULL) return NULL;

    size_t pad = width - len;
    memset(result, ' ', pad);
    memcpy(result + pad, s, len);
    result[width] = '\0';

    return result;
}

// Zero-fill str on the left to given width
sds str_zfill(const char* s, int64_t width) {
    size_t len = strlen(s);
    if ((size_t)width <= len) return sdsnew(s);

    sds result = sdsnewlen(NULL, width);
    if (result == NULL) return NULL;

    size_t pad = width - len;

    // Handle sign
    size_t start = 0;
    if (len > 0 && (s[0] == '+' || s[0] == '-')) {
        result[0] = s[0];
        start = 1;
        memset(result + 1, '0', pad);
    } else {
        memset(result, '0', pad);
    }

    memcpy(result + pad + start, s + start, len - start);
    result[width] = '\0';

    return result;
}

// ============================================================================
// Type Conversion Functions
// ============================================================================

// Convert int to string
sds int_to_str(int64_t n) {
    char buf[32];
    snprintf(buf, sizeof(buf), "%ld", (long)n);
    return sdsnew(buf);
}

// Convert float to string
sds float_to_str(double f) {
    char buf[64];
    snprintf(buf, sizeof(buf), "%g", f);
    // Python always shows decimal point for floats: str(1.0) -> "1.0", not "1"
    // Check if result contains '.' or 'e'/'E' (scientific notation)
    int has_decimal = 0;
    for (int i = 0; buf[i] != '\0'; i++) {
        if (buf[i] == '.' || buf[i] == 'e' || buf[i] == 'E') {
            has_decimal = 1;
            break;
        }
    }
    if (!has_decimal) {
        // Append ".0" to make it look like a float
        size_t len = strlen(buf);
        buf[len] = '.';
        buf[len+1] = '0';
        buf[len+2] = '\0';
    }
    return sdsnew(buf);
}

// Convert bool to string
sds bool_to_str(int64_t b) {
    return sdsnew(b ? "True" : "False");
}

// Convert int to binary string (0b prefix)
sds int_to_bin(int64_t n) {
    char buf[67]; // 64 bits + "0b" + null
    char *p = buf + sizeof(buf) - 1;
    *p = '\0';

    int negative = n < 0;
    uint64_t val = negative ? (uint64_t)(-n) : (uint64_t)n;

    if (val == 0) {
        *--p = '0';
    } else {
        while (val > 0) {
            *--p = '0' + (val & 1);
            val >>= 1;
        }
    }

    *--p = 'b';
    *--p = negative ? '-' : '0';
    if (negative) *--p = '-';
    else {
        // Put 0b back correctly
        p++;  // Undo the '-' we didn't use
    }

    // Fix: Rebuild properly
    p = buf + sizeof(buf) - 1;
    *p = '\0';
    val = negative ? (uint64_t)(-n) : (uint64_t)n;

    if (val == 0) {
        *--p = '0';
    } else {
        while (val > 0) {
            *--p = '0' + (val & 1);
            val >>= 1;
        }
    }
    *--p = 'b';
    *--p = '0';
    if (negative) *--p = '-';

    return sdsnew(p);
}

// Convert int to hexadecimal string (0x prefix)
sds int_to_hex(int64_t n) {
    char buf[24];
    if (n < 0) {
        snprintf(buf, sizeof(buf), "-0x%lx", (unsigned long)(-n));
    } else {
        snprintf(buf, sizeof(buf), "0x%lx", (unsigned long)n);
    }
    return sdsnew(buf);
}

// Convert int to octal string (0o prefix)
sds int_to_oct(int64_t n) {
    char buf[24];
    if (n < 0) {
        snprintf(buf, sizeof(buf), "-0o%lo", (unsigned long)(-n));
    } else {
        snprintf(buf, sizeof(buf), "0o%lo", (unsigned long)n);
    }
    return sdsnew(buf);
}

// Convert int to single character (chr)
sds int_to_chr(int64_t n) {
    if (n < 0 || n > 0x10FFFF) {
        return sdsempty();  // Invalid code point
    }
    char buf[5];
    if (n < 0x80) {
        buf[0] = (char)n;
        buf[1] = '\0';
    } else if (n < 0x800) {
        buf[0] = 0xC0 | (n >> 6);
        buf[1] = 0x80 | (n & 0x3F);
        buf[2] = '\0';
    } else if (n < 0x10000) {
        buf[0] = 0xE0 | (n >> 12);
        buf[1] = 0x80 | ((n >> 6) & 0x3F);
        buf[2] = 0x80 | (n & 0x3F);
        buf[3] = '\0';
    } else {
        buf[0] = 0xF0 | (n >> 18);
        buf[1] = 0x80 | ((n >> 12) & 0x3F);
        buf[2] = 0x80 | ((n >> 6) & 0x3F);
        buf[3] = 0x80 | (n & 0x3F);
        buf[4] = '\0';
    }
    return sdsnew(buf);
}

// Get ordinal value of single character (ord)
int64_t str_ord(const char* s) {
    if (s == NULL || s[0] == '\0') return -1;
    unsigned char c = (unsigned char)s[0];
    if (c < 0x80) {
        return c;
    }
    // Handle UTF-8 multibyte
    if ((c & 0xE0) == 0xC0 && s[1]) {
        return ((c & 0x1F) << 6) | (s[1] & 0x3F);
    }
    if ((c & 0xF0) == 0xE0 && s[1] && s[2]) {
        return ((c & 0x0F) << 12) | ((s[1] & 0x3F) << 6) | (s[2] & 0x3F);
    }
    if ((c & 0xF8) == 0xF0 && s[1] && s[2] && s[3]) {
        return ((c & 0x07) << 18) | ((s[1] & 0x3F) << 12) | ((s[2] & 0x3F) << 6) | (s[3] & 0x3F);
    }
    return c;  // Fallback to byte value
}

// ============================================================================
// ASCII Representation Functions
// ============================================================================

// Convert int to ascii representation (same as str for ints)
sds int_to_ascii(int64_t n) {
    return int_to_str(n);
}

// Convert float to ascii representation
sds float_to_ascii(double f) {
    return float_to_str(f);
}

// Convert bool to ascii representation
sds bool_to_ascii(int64_t b) {
    return bool_to_str(b);
}

// Convert str to ascii representation (with escapes and quotes)
sds str_to_ascii(const char* s) {
    size_t len = strlen(s);
    sds result = sdsnewlen(NULL, len * 4 + 3);  // Worst case: all escapes + quotes
    if (result == NULL) return NULL;

    char *p = result;
    *p++ = '\'';
    for (size_t i = 0; i < len; i++) {
        unsigned char c = (unsigned char)s[i];
        if (c == '\\') {
            *p++ = '\\'; *p++ = '\\';
        } else if (c == '\'') {
            *p++ = '\\'; *p++ = '\'';
        } else if (c == '\n') {
            *p++ = '\\'; *p++ = 'n';
        } else if (c == '\r') {
            *p++ = '\\'; *p++ = 'r';
        } else if (c == '\t') {
            *p++ = '\\'; *p++ = 't';
        } else if (c >= 32 && c < 127) {
            *p++ = c;
        } else {
            p += sprintf(p, "\\x%02x", c);
        }
    }
    *p++ = '\'';
    *p = '\0';
    sdssetlen(result, p - result);
    return result;
}

// Convert bytes to ascii representation
sds bytes_to_ascii(const char* s) {
    sds inner = str_to_ascii(s);
    sds result = sdsnewlen("b", 1);
    result = sdscatsds(result, inner);
    sdsfree(inner);
    return result;
}

// Convert None to ascii representation
sds none_to_ascii(void) {
    return sdsnew("None");
}

// ============================================================================
// Sequence Functions
// ============================================================================

// Return reversed string
sds str_reversed(const char* s) {
    return str_reverse(s);  // Already implemented above
}

// Forward declaration for list
typedef struct {
    int64_t len;
    int64_t capacity;
    int64_t* data;
} PyListSort;

// External list function
extern PyListSort* list_with_capacity(int64_t capacity);

// Helper for int comparison in qsort
static int cmp_int64_sort(const void* a, const void* b) {
    int64_t va = *(const int64_t*)a;
    int64_t vb = *(const int64_t*)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

// Return sorted list of character ordinal values
PyListSort* str_sorted(const char* s) {
    if (s == NULL || strlen(s) == 0) {
        PyListSort* result = list_with_capacity(0);
        if (result) result->len = 0;
        return result;
    }

    size_t len = strlen(s);
    PyListSort* result = list_with_capacity((int64_t)len);
    if (result == NULL) return NULL;

    // Convert characters to ordinal values
    for (size_t i = 0; i < len; i++) {
        result->data[i] = (int64_t)(unsigned char)s[i];
    }
    result->len = (int64_t)len;

    // Sort the ordinal values
    qsort(result->data, len, sizeof(int64_t), cmp_int64_sort);

    return result;
}

// ============================================================================
// String Repetition
// ============================================================================

// Repeat string n times: "abc" * 3 -> "abcabcabc"
sds str_repeat(const char* s, int64_t n) {
    if (s == NULL || n <= 0) return sdsempty();

    size_t len = strlen(s);
    if (len == 0) return sdsempty();

    sds result = sdsnewlen(NULL, len * n);
    if (result == NULL) return sdsempty();

    for (int64_t i = 0; i < n; i++) {
        memcpy(result + (i * len), s, len);
    }
    result[len * n] = '\0';

    return result;
}

// ============================================================================
// Min/Max Functions
// ============================================================================

// Return minimum character in string (as single-char string)
sds str_min(const char* s) {
    size_t len = strlen(s);
    if (len == 0) return sdsempty();  // Empty string

    unsigned char min_char = (unsigned char)s[0];
    for (size_t i = 1; i < len; i++) {
        unsigned char c = (unsigned char)s[i];
        if (c < min_char) min_char = c;
    }

    char buf[2] = {min_char, '\0'};
    return sdsnew(buf);
}

// Return maximum character in string (as single-char string)
sds str_max(const char* s) {
    size_t len = strlen(s);
    if (len == 0) return sdsempty();  // Empty string

    unsigned char max_char = (unsigned char)s[0];
    for (size_t i = 1; i < len; i++) {
        unsigned char c = (unsigned char)s[i];
        if (c > max_char) max_char = c;
    }

    char buf[2] = {max_char, '\0'};
    return sdsnew(buf);
}

// ============================================================================
// str() conversion functions
// ============================================================================

// Forward declarations for list
typedef struct {
    int64_t len;
    int64_t capacity;
    int64_t* data;
} PyListStr;

// Forward declarations for set
#define SET_OCCUPIED_STR 1
typedef struct {
    int64_t key;
    uint8_t state;
} SetEntryStr;
typedef struct {
    int64_t len;
    int64_t capacity;
    SetEntryStr* entries;
} PySetStr;

// Forward declarations for dict
#define DICT_OCCUPIED_STR 1
typedef struct {
    int64_t key;
    int64_t value;
    uint8_t state;
} DictEntryStr;
typedef struct {
    int64_t len;
    int64_t capacity;
    DictEntryStr* entries;
} PyDictStr;

// Convert bytes to str (returns b'...' representation)
sds bytes_to_str(const char* bytes) {
    if (bytes == NULL) return sdsnew("b''");

    size_t len = strlen(bytes);
    sds result = sdsnew("b'");
    result = sdscatlen(result, bytes, len);
    result = sdscat(result, "'");
    return result;
}

// Convert None to str
sds none_to_str(void) {
    return sdsnew("None");
}

// Helper for int comparison in qsort
static int cmp_int64_str(const void* a, const void* b) {
    int64_t va = *(const int64_t*)a;
    int64_t vb = *(const int64_t*)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

// Convert list to str
sds list_to_str(PyListStr* list) {
    if (list == NULL || list->len == 0) return sdsnew("[]");

    sds result = sdsnew("[");
    for (int64_t i = 0; i < list->len; i++) {
        if (i > 0) result = sdscat(result, ", ");
        char buf[32];
        snprintf(buf, sizeof(buf), "%lld", (long long)list->data[i]);
        result = sdscat(result, buf);
    }
    result = sdscat(result, "]");
    return result;
}

// Convert set to str
sds set_to_str(PySetStr* set) {
    if (set == NULL || set->len == 0) return sdsnew("set()");

    // Collect elements
    int64_t* elements = malloc(set->len * sizeof(int64_t));
    if (elements == NULL) return sdsnew("set()");

    int64_t j = 0;
    for (int64_t i = 0; i < set->capacity && j < set->len; i++) {
        if (set->entries[i].state == SET_OCCUPIED_STR) {
            elements[j++] = set->entries[i].key;
        }
    }

    // Sort elements for consistent output
    qsort(elements, j, sizeof(int64_t), cmp_int64_str);

    sds result = sdsnew("{");
    for (int64_t i = 0; i < j; i++) {
        if (i > 0) result = sdscat(result, ", ");
        char buf[32];
        snprintf(buf, sizeof(buf), "%lld", (long long)elements[i]);
        result = sdscat(result, buf);
    }
    result = sdscat(result, "}");

    free(elements);
    return result;
}

// Convert dict to str
sds dict_to_str(PyDictStr* dict) {
    if (dict == NULL || dict->len == 0) return sdsnew("{}");

    // Collect key-value pairs
    typedef struct {
        int64_t key;
        int64_t value;
    } KVPair;
    KVPair* pairs = malloc(dict->len * sizeof(KVPair));
    if (pairs == NULL) return sdsnew("{}");

    int64_t j = 0;
    for (int64_t i = 0; i < dict->capacity && j < dict->len; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED_STR) {
            pairs[j].key = dict->entries[i].key;
            pairs[j].value = dict->entries[i].value;
            j++;
        }
    }

    // Sort by key for consistent output
    qsort(pairs, j, sizeof(KVPair), cmp_int64_str);

    sds result = sdsnew("{");
    for (int64_t i = 0; i < j; i++) {
        if (i > 0) result = sdscat(result, ", ");
        char buf[64];
        snprintf(buf, sizeof(buf), "%lld: %lld", (long long)pairs[i].key, (long long)pairs[i].value);
        result = sdscat(result, buf);
    }
    result = sdscat(result, "}");

    free(pairs);
    return result;
}

// Convert string-keyed dict to str
typedef struct {
    char* key;
    int64_t value;
    uint8_t state;
} StrDictEntryStr;
typedef struct {
    int64_t len;
    int64_t capacity;
    StrDictEntryStr* entries;
} PyStrDictStr;

// Helper for string comparison in qsort
static int cmp_str_entry(const void* a, const void* b) {
    const StrDictEntryStr* ea = a;
    const StrDictEntryStr* eb = b;
    return strcmp(ea->key, eb->key);
}

sds str_dict_to_str(PyStrDictStr* dict) {
    if (dict == NULL || dict->len == 0) return sdsnew("{}");

    // Collect entries
    StrDictEntryStr* entries = malloc(dict->len * sizeof(StrDictEntryStr));
    if (entries == NULL) return sdsnew("{}");

    int64_t j = 0;
    for (int64_t i = 0; i < dict->capacity && j < dict->len; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED_STR) {
            entries[j] = dict->entries[i];
            j++;
        }
    }

    // Sort by key for consistent output
    qsort(entries, j, sizeof(StrDictEntryStr), cmp_str_entry);

    sds result = sdsnew("{");
    for (int64_t i = 0; i < j; i++) {
        if (i > 0) result = sdscat(result, ", ");
        // Format "'key': value"
        result = sdscat(result, "'");
        result = sdscat(result, entries[i].key);
        result = sdscat(result, "': ");
        char buf[32];
        snprintf(buf, sizeof(buf), "%lld", (long long)entries[i].value);
        result = sdscat(result, buf);
    }
    result = sdscat(result, "}");

    free(entries);
    return result;
}

// ============================================================================
// String Format Functions (% operator)
// ============================================================================

// Format string with integer argument: "value: %d" % 42 -> "value: 42"
sds str_format_int(const char* fmt, int64_t arg) {
    // Simple implementation: just replace the first % format specifier with the int
    // This is a simplified version of Python's % formatting
    size_t fmtlen = strlen(fmt);

    // Find the first % that's not %%
    const char* p = fmt;
    const char* spec_start = NULL;
    while (*p) {
        if (*p == '%') {
            if (p[1] == '%') {
                p += 2;  // Skip %%
            } else {
                spec_start = p;
                break;
            }
        } else {
            p++;
        }
    }

    if (spec_start == NULL) {
        // No format specifier found, just return the format string
        return sdsnew(fmt);
    }

    // Find the end of the format specifier (d, i, x, X, o, b, s, etc.)
    p = spec_start + 1;
    while (*p && (isdigit(*p) || *p == '-' || *p == '+' || *p == ' ' || *p == '#' || *p == '0' || *p == '.')) {
        p++;
    }
    char spec_char = *p ? *p : 'd';  // Default to 'd'
    p++;  // Move past the specifier char

    // Build result
    size_t prefix_len = spec_start - fmt;
    size_t suffix_len = fmtlen - (p - fmt);

    char buf[64];
    switch (spec_char) {
        case 'd': case 'i':
            snprintf(buf, sizeof(buf), "%lld", (long long)arg);
            break;
        case 'x':
            snprintf(buf, sizeof(buf), "%llx", (unsigned long long)arg);
            break;
        case 'X':
            snprintf(buf, sizeof(buf), "%llX", (unsigned long long)arg);
            break;
        case 'o':
            snprintf(buf, sizeof(buf), "%llo", (unsigned long long)arg);
            break;
        case 'b':
            // Binary - not supported by printf, do it manually
            {
                char *bp = buf + sizeof(buf) - 1;
                *bp = '\0';
                uint64_t val = arg < 0 ? (uint64_t)(-arg) : (uint64_t)arg;
                if (val == 0) {
                    *--bp = '0';
                } else {
                    while (val > 0) {
                        *--bp = '0' + (val & 1);
                        val >>= 1;
                    }
                }
                if (arg < 0) *--bp = '-';
                memmove(buf, bp, strlen(bp) + 1);
            }
            break;
        case 's':
            // Convert int to string
            snprintf(buf, sizeof(buf), "%lld", (long long)arg);
            break;
        default:
            snprintf(buf, sizeof(buf), "%lld", (long long)arg);
            break;
    }

    size_t buflen = strlen(buf);
    sds result = sdsnewlen(NULL, prefix_len + buflen + suffix_len);
    if (result == NULL) return NULL;

    memcpy(result, fmt, prefix_len);
    memcpy(result + prefix_len, buf, buflen);
    memcpy(result + prefix_len + buflen, p, suffix_len);
    result[prefix_len + buflen + suffix_len] = '\0';

    return result;
}

// Format string with float argument
sds str_format_float(const char* fmt, double arg) {
    size_t fmtlen = strlen(fmt);

    const char* p = fmt;
    const char* spec_start = NULL;
    while (*p) {
        if (*p == '%') {
            if (p[1] == '%') {
                p += 2;
            } else {
                spec_start = p;
                break;
            }
        } else {
            p++;
        }
    }

    if (spec_start == NULL) {
        return sdsnew(fmt);
    }

    p = spec_start + 1;
    while (*p && (isdigit(*p) || *p == '-' || *p == '+' || *p == ' ' || *p == '#' || *p == '0' || *p == '.')) {
        p++;
    }
    char spec_char = *p ? *p : 'g';
    p++;

    size_t prefix_len = spec_start - fmt;
    size_t suffix_len = fmtlen - (p - fmt);

    char buf[64];
    switch (spec_char) {
        case 'f': case 'F':
            snprintf(buf, sizeof(buf), "%f", arg);
            break;
        case 'e':
            snprintf(buf, sizeof(buf), "%e", arg);
            break;
        case 'E':
            snprintf(buf, sizeof(buf), "%E", arg);
            break;
        case 'g':
            snprintf(buf, sizeof(buf), "%g", arg);
            break;
        case 'G':
            snprintf(buf, sizeof(buf), "%G", arg);
            break;
        case 's':
            snprintf(buf, sizeof(buf), "%g", arg);
            break;
        default:
            snprintf(buf, sizeof(buf), "%g", arg);
            break;
    }

    size_t buflen = strlen(buf);
    sds result = sdsnewlen(NULL, prefix_len + buflen + suffix_len);
    if (result == NULL) return NULL;

    memcpy(result, fmt, prefix_len);
    memcpy(result + prefix_len, buf, buflen);
    memcpy(result + prefix_len + buflen, p, suffix_len);
    result[prefix_len + buflen + suffix_len] = '\0';

    return result;
}

// Format string with string argument
sds str_format_str(const char* fmt, const char* arg) {
    size_t fmtlen = strlen(fmt);

    const char* p = fmt;
    const char* spec_start = NULL;
    while (*p) {
        if (*p == '%') {
            if (p[1] == '%') {
                p += 2;
            } else {
                spec_start = p;
                break;
            }
        } else {
            p++;
        }
    }

    if (spec_start == NULL) {
        return sdsnew(fmt);
    }

    p = spec_start + 1;
    while (*p && (isdigit(*p) || *p == '-' || *p == '+' || *p == ' ' || *p == '#' || *p == '0' || *p == '.')) {
        p++;
    }
    p++;  // Skip specifier char

    size_t prefix_len = spec_start - fmt;
    size_t suffix_len = fmtlen - (p - fmt);
    size_t arglen = strlen(arg);

    sds result = sdsnewlen(NULL, prefix_len + arglen + suffix_len);
    if (result == NULL) return NULL;

    memcpy(result, fmt, prefix_len);
    memcpy(result + prefix_len, arg, arglen);
    memcpy(result + prefix_len + arglen, p, suffix_len);
    result[prefix_len + arglen + suffix_len] = '\0';

    return result;
}

// Format string with bool argument (True/False)
sds str_format_bool(const char* fmt, int64_t arg) {
    const char* val = arg ? "True" : "False";
    return str_format_str(fmt, val);
}

// Format string with bytes argument (uses repr like b'...')
sds str_format_bytes(const char* fmt, const char* arg) {
    sds bytes_repr = bytes_to_str(arg);
    sds result = str_format_str(fmt, bytes_repr);
    sdsfree(bytes_repr);
    return result;
}

// Format bytes with bytes argument (uses raw bytes)
sds bytes_format_bytes(const char* fmt, const char* arg) {
    return str_format_str(fmt, arg);
}

// Format string with None argument
sds str_format_none(const char* fmt) {
    return str_format_str(fmt, "None");
}

// Format string with list argument
sds str_format_list(const char* fmt, PyListStr* list) {
    sds list_repr = list_to_str(list);
    sds result = str_format_str(fmt, list_repr);
    sdsfree(list_repr);
    return result;
}

// Format string with dict argument
sds str_format_dict(const char* fmt, PyDictStr* dict) {
    sds dict_repr = dict_to_str(dict);
    sds result = str_format_str(fmt, dict_repr);
    sdsfree(dict_repr);
    return result;
}

// Format string with set argument
sds str_format_set(const char* fmt, PySetStr* set) {
    sds set_repr = set_to_str(set);
    sds result = str_format_str(fmt, set_repr);
    sdsfree(set_repr);
    return result;
}

// ============================================================================
// String Iteration Support
// ============================================================================

// Get character at index as a new single-character string
// Returns NULL if index out of bounds
sds str_char_at(const char* s, int64_t index) {
    if (s == NULL) return NULL;
    size_t len = strlen(s);
    if (index < 0) index = len + index;
    if (index < 0 || (size_t)index >= len) return NULL;
    return sdsnewlen(s + index, 1);
}

// ============================================================================
// any() and all() builtins for strings
// ============================================================================

// any(str) - returns true if any character is non-zero (always true for non-empty strings)
// In Python, any character has a truthy value
int64_t str_any(const char* s) {
    if (s == NULL || s[0] == '\0') return 0;
    // Any non-empty string has truthy characters
    return 1;
}

// all(str) - returns true if all characters are truthy (always true for non-empty strings)
// Empty string returns True for all()
int64_t str_all(const char* s) {
    // Empty string or NULL returns True for all()
    return 1;
}

// ============================================================================
// repr() builtin - returns string representation of an object
// ============================================================================

// repr(int) - returns string like "42"
sds repr_int(int64_t value) {
    char buf[32];
    snprintf(buf, sizeof(buf), "%lld", (long long)value);
    return sdsnew(buf);
}

// repr(float) - returns string like "3.14"
sds repr_float(double value) {
    char buf[64];
    snprintf(buf, sizeof(buf), "%g", value);
    return sdsnew(buf);
}

// repr(bool) - returns "True" or "False"
sds repr_bool(int64_t value) {
    return sdsnew(value ? "True" : "False");
}

// repr(str) - returns string with quotes like "'hello'"
sds repr_str(const char* s) {
    if (s == NULL) return sdsnew("''");
    sds result = sdsnew("'");
    result = sdscat(result, s);
    result = sdscat(result, "'");
    return result;
}

// repr(None) - returns "None"
sds repr_none(void) {
    return sdsnew("None");
}
