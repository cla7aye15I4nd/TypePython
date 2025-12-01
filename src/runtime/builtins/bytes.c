// TypePython Runtime Library - Bytes Module
// Builtin string/bytes operations using SDS (Simple Dynamic Strings)
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
// TypePython Bytes API (exported functions)
// ============================================================================

// Note: Input bytes can be either raw C strings (from literals) or SDS strings.
// For raw C strings we use strlen() to get length.
// Output is always a proper SDS string.

// Helper to get length - works with both raw C strings and SDS
// For raw C strings (from literals), we use strlen
// For SDS strings, the length is stored in the header
static inline size_t bytes_getlen(const char* s) {
    if (s == NULL) return 0;
    // Just use strlen - it works for both raw C strings and SDS
    // (SDS strings are null-terminated too)
    return strlen(s);
}

// Create new bytes from C string literal
sds bytes_new(const char* s) {
    return sdsnew(s);
}

// Create new bytes with specified length
sds bytes_newlen(const char* s, int64_t len) {
    return sdsnewlen(s, (size_t)len);
}

// Create empty bytes
sds bytes_empty(void) {
    return sdsempty();
}

// Free bytes
void bytes_free(sds s) {
    sdsfree(s);
}

// Get length of bytes (works with raw C strings and SDS)
int64_t bytes_len(const char* s) {
    if (s == NULL) return 0;
    return (int64_t)strlen(s);
}

// Duplicate bytes
sds bytes_dup(const char* s) {
    return sdsnew(s);
}

// Concatenate two bytes - returns new sds
// Works with raw C strings or SDS strings as input
sds strcat_bytes(const char* s1, const char* s2) {
    size_t len1 = strlen(s1);
    size_t len2 = strlen(s2);
    sds result = sdsnewlen(NULL, len1 + len2);
    if (result == NULL) return NULL;
    memcpy(result, s1, len1);
    memcpy(result + len1, s2, len2);
    result[len1 + len2] = '\0';
    return result;
}

// Compare two bytes - returns 1 if equal, 0 if not
// Works with raw C strings or SDS strings
int64_t strcmp_bytes(const char* s1, const char* s2) {
    return strcmp(s1, s2) == 0 ? 1 : 0;
}

// Compare two bytes - returns -1, 0, or 1
int64_t bytes_cmp(const char* s1, const char* s2) {
    int cmp = strcmp(s1, s2);
    if (cmp < 0) return -1;
    if (cmp > 0) return 1;
    return 0;
}

// Less than comparison
int64_t bytes_lt(const char* s1, const char* s2) {
    return strcmp(s1, s2) < 0 ? 1 : 0;
}

// Less than or equal comparison
int64_t bytes_le(const char* s1, const char* s2) {
    return strcmp(s1, s2) <= 0 ? 1 : 0;
}

// Greater than comparison
int64_t bytes_gt(const char* s1, const char* s2) {
    return strcmp(s1, s2) > 0 ? 1 : 0;
}

// Greater than or equal comparison
int64_t bytes_ge(const char* s1, const char* s2) {
    return strcmp(s1, s2) >= 0 ? 1 : 0;
}

// Repeat bytes n times
sds strrepeat_bytes(const char* s, int64_t n) {
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
int64_t bytes_getitem(const char* s, int64_t index) {
    size_t len = strlen(s);
    if (index < 0) index = len + index;
    if (index < 0 || (size_t)index >= len) return -1;
    return (unsigned char)s[index];
}

// Get slice of bytes (start:end)
sds bytes_slice(const char* s, int64_t start, int64_t end) {
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

// Get slice of bytes with step (start:end:step)
// INT64_MAX is used as sentinel for "use default value"
sds bytes_slice_step(const char* s, int64_t start, int64_t end, int64_t step) {
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

// Check if bytes contains substring
int64_t bytes_contains(const char* haystack, const char* needle) {
    if (strlen(needle) == 0) return 1;
    return strstr(haystack, needle) != NULL ? 1 : 0;
}

// Find first occurrence of substring (returns -1 if not found)
int64_t bytes_find(const char* haystack, const char* needle) {
    if (strlen(needle) == 0) return 0;
    char *pos = strstr(haystack, needle);
    if (pos == NULL) return -1;
    return pos - haystack;
}

// Check if bytes starts with prefix
int64_t bytes_startswith(const char* s, const char* prefix) {
    size_t slen = strlen(s);
    size_t plen = strlen(prefix);
    if (plen > slen) return 0;
    return memcmp(s, prefix, plen) == 0 ? 1 : 0;
}

// Check if bytes ends with suffix
int64_t bytes_endswith(const char* s, const char* suffix) {
    size_t slen = strlen(s);
    size_t suflen = strlen(suffix);
    if (suflen > slen) return 0;
    return memcmp(s + slen - suflen, suffix, suflen) == 0 ? 1 : 0;
}

// Convert to uppercase (returns new bytes)
sds bytes_upper(const char* s) {
    sds result = sdsnew(s);
    if (result == NULL) return NULL;
    sdstoupper(result);
    return result;
}

// Convert to lowercase (returns new bytes)
sds bytes_lower(const char* s) {
    sds result = sdsnew(s);
    if (result == NULL) return NULL;
    sdstolower(result);
    return result;
}

// Capitalize: first character uppercase, rest lowercase (returns new bytes)
sds bytes_capitalize(const char* s) {
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

// Title case: uppercase after whitespace, lowercase elsewhere (returns new bytes)
sds bytes_title(const char* s) {
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

// Swap case: uppercase to lowercase and vice versa (returns new bytes)
sds bytes_swapcase(const char* s) {
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

// Helper: check if character is in chars string
static int char_in_str(char c, const char* chars) {
    while (*chars) {
        if (*chars == c) return 1;
        chars++;
    }
    return 0;
}

// Strip characters from both ends (returns new bytes)
// If chars is NULL or empty, strips whitespace
sds bytes_strip(const char* s, const char* chars) {
    if (chars == NULL || *chars == '\0') {
        sds result = sdsnew(s);
        if (result == NULL) return NULL;
        sdstrim(result, " \t\n\r\f\v");
        return result;
    }
    size_t len = strlen(s);
    const char *start = s;
    const char *end = s + len - 1;
    while (start <= end && char_in_str(*start, chars)) start++;
    while (end >= start && char_in_str(*end, chars)) end--;
    return sdsnewlen(start, end - start + 1);
}

// Strip characters from left (returns new bytes)
sds bytes_lstrip(const char* s, const char* chars) {
    size_t len = strlen(s);
    const char *start = s;
    if (chars == NULL || *chars == '\0') {
        while (start < s + len && isspace(*start)) start++;
    } else {
        while (start < s + len && char_in_str(*start, chars)) start++;
    }
    return sdsnewlen(start, s + len - start);
}

// Strip characters from right (returns new bytes)
sds bytes_rstrip(const char* s, const char* chars) {
    size_t len = strlen(s);
    const char *end = s + len - 1;
    if (chars == NULL || *chars == '\0') {
        while (end >= s && isspace(*end)) end--;
    } else {
        while (end >= s && char_in_str(*end, chars)) end--;
    }
    return sdsnewlen(s, end - s + 1);
}

// Replace all occurrences of old with new (returns new bytes)
sds bytes_replace(const char* s, const char* old, const char* new_str) {
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
int64_t bytes_count(const char* s, const char* sub) {
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

// Join array of bytes with separator
sds bytes_join(const char* sep, char **parts, int64_t count) {
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
int64_t bytes_isalnum(const char* s) {
    size_t len = strlen(s);
    if (len == 0) return 0;
    for (size_t i = 0; i < len; i++) {
        if (!isalnum((unsigned char)s[i])) return 0;
    }
    return 1;
}

// Check if all characters are alphabetic
int64_t bytes_isalpha(const char* s) {
    size_t len = strlen(s);
    if (len == 0) return 0;
    for (size_t i = 0; i < len; i++) {
        if (!isalpha((unsigned char)s[i])) return 0;
    }
    return 1;
}

// Check if all characters are digits
int64_t bytes_isdigit(const char* s) {
    size_t len = strlen(s);
    if (len == 0) return 0;
    for (size_t i = 0; i < len; i++) {
        if (!isdigit((unsigned char)s[i])) return 0;
    }
    return 1;
}

// Check if all characters are whitespace
int64_t bytes_isspace(const char* s) {
    size_t len = strlen(s);
    if (len == 0) return 0;
    for (size_t i = 0; i < len; i++) {
        if (!isspace((unsigned char)s[i])) return 0;
    }
    return 1;
}

// Check if all characters are lowercase
int64_t bytes_islower(const char* s) {
    size_t len = strlen(s);
    int has_cased = 0;
    for (size_t i = 0; i < len; i++) {
        if (isupper((unsigned char)s[i])) return 0;
        if (islower((unsigned char)s[i])) has_cased = 1;
    }
    return has_cased ? 1 : 0;
}

// Check if all characters are uppercase
int64_t bytes_isupper(const char* s) {
    size_t len = strlen(s);
    int has_cased = 0;
    for (size_t i = 0; i < len; i++) {
        if (islower((unsigned char)s[i])) return 0;
        if (isupper((unsigned char)s[i])) has_cased = 1;
    }
    return has_cased ? 1 : 0;
}

// Reverse bytes (returns new bytes)
sds bytes_reverse(const char* s) {
    size_t len = strlen(s);
    sds result = sdsnewlen(NULL, len);
    if (result == NULL) return NULL;

    for (size_t i = 0; i < len; i++) {
        result[i] = s[len - 1 - i];
    }
    result[len] = '\0';
    return result;
}

// Center bytes in field of given width with optional fill character
// fillchar should be a single-byte string, uses first char or space if NULL/empty
sds bytes_center(const char* s, int64_t width, const char* fillchar) {
    size_t len = strlen(s);
    if ((size_t)width <= len) return sdsnew(s);

    char fill = (fillchar != NULL && *fillchar != '\0') ? fillchar[0] : ' ';
    size_t total_pad = width - len;
    size_t left_pad = total_pad / 2;
    size_t right_pad = total_pad - left_pad;

    sds result = sdsnewlen(NULL, width);
    if (result == NULL) return NULL;

    memset(result, fill, left_pad);
    memcpy(result + left_pad, s, len);
    memset(result + left_pad + len, fill, right_pad);
    result[width] = '\0';

    return result;
}

// Left-justify bytes in field of given width with optional fill character
sds bytes_ljust(const char* s, int64_t width, const char* fillchar) {
    size_t len = strlen(s);
    if ((size_t)width <= len) return sdsnew(s);

    char fill = (fillchar != NULL && *fillchar != '\0') ? fillchar[0] : ' ';
    sds result = sdsnewlen(NULL, width);
    if (result == NULL) return NULL;

    memcpy(result, s, len);
    memset(result + len, fill, width - len);
    result[width] = '\0';

    return result;
}

// Right-justify bytes in field of given width with optional fill character
sds bytes_rjust(const char* s, int64_t width, const char* fillchar) {
    size_t len = strlen(s);
    if ((size_t)width <= len) return sdsnew(s);

    char fill = (fillchar != NULL && *fillchar != '\0') ? fillchar[0] : ' ';
    sds result = sdsnewlen(NULL, width);
    if (result == NULL) return NULL;

    size_t pad = width - len;
    memset(result, fill, pad);
    memcpy(result + pad, s, len);
    result[width] = '\0';

    return result;
}

// Zero-fill bytes on the left to given width
sds bytes_zfill(const char* s, int64_t width) {
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
// Builtin Functions: sum
// ============================================================================

// Sum all bytes in the bytes object
int64_t bytes_sum(const char* s, int64_t start) {
    int64_t sum = start;
    if (s != NULL) {
        size_t len = strlen(s);
        for (size_t i = 0; i < len; i++) {
            sum += (unsigned char)s[i];
        }
    }
    return sum;
}

// ============================================================================
// Builtin Functions: min/max
// ============================================================================

// Return minimum byte value in bytes object
int64_t bytes_min(const char* s) {
    if (s == NULL || s[0] == '\0') return 0;  // Empty bytes

    size_t len = strlen(s);
    unsigned char min_byte = (unsigned char)s[0];
    for (size_t i = 1; i < len; i++) {
        unsigned char c = (unsigned char)s[i];
        if (c < min_byte) min_byte = c;
    }
    return (int64_t)min_byte;
}

// Return maximum byte value in bytes object
int64_t bytes_max(const char* s) {
    if (s == NULL || s[0] == '\0') return 0;  // Empty bytes

    size_t len = strlen(s);
    unsigned char max_byte = (unsigned char)s[0];
    for (size_t i = 1; i < len; i++) {
        unsigned char c = (unsigned char)s[i];
        if (c > max_byte) max_byte = c;
    }
    return (int64_t)max_byte;
}

// ============================================================================
// Sorted/Reversed Functions
// ============================================================================

// Forward declaration for list
typedef struct {
    int64_t len;
    int64_t capacity;
    int64_t* data;
} PyListBytes;

// External list function
extern PyListBytes* list_with_capacity(int64_t capacity);

// Helper for int comparison in qsort
static int cmp_int64_bytes(const void* a, const void* b) {
    int64_t va = *(const int64_t*)a;
    int64_t vb = *(const int64_t*)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

// Return sorted list of byte values
PyListBytes* bytes_sorted(const char* s) {
    if (s == NULL || strlen(s) == 0) {
        PyListBytes* result = list_with_capacity(0);
        if (result) result->len = 0;
        return result;
    }

    size_t len = strlen(s);
    PyListBytes* result = list_with_capacity((int64_t)len);
    if (result == NULL) return NULL;

    // Convert bytes to values
    for (size_t i = 0; i < len; i++) {
        result->data[i] = (int64_t)(unsigned char)s[i];
    }
    result->len = (int64_t)len;

    // Sort
    qsort(result->data, len, sizeof(int64_t), cmp_int64_bytes);

    return result;
}

// Return reversed bytes
sds bytes_reversed(const char* s) {
    if (s == NULL) return sdsempty();

    size_t len = strlen(s);
    if (len == 0) return sdsempty();

    sds result = sdsnewlen(NULL, len);
    if (result == NULL) return NULL;

    for (size_t i = 0; i < len; i++) {
        result[i] = s[len - 1 - i];
    }
    result[len] = '\0';

    return result;
}

// Repeat bytes n times: b"abc" * 3 -> b"abcabcabc"
sds bytes_repeat(const char* s, int64_t n) {
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

// Check if a byte value is contained in bytes: 104 in b"hello" -> 1
int64_t bytes_contains_byte(const char* s, int64_t byte_value) {
    if (s == NULL) return 0;

    unsigned char target = (unsigned char)byte_value;
    size_t len = strlen(s);

    for (size_t i = 0; i < len; i++) {
        if ((unsigned char)s[i] == target) return 1;
    }
    return 0;
}

// ============================================================================
// any() and all() builtins for bytes
// ============================================================================

// any(bytes) - returns true if any byte is non-zero
int64_t bytes_any(const char* s) {
    if (s == NULL) return 0;
    size_t len = strlen(s);
    if (len == 0) return 0;
    for (size_t i = 0; i < len; i++) {
        if ((unsigned char)s[i] != 0) return 1;
    }
    return 0;
}

// all(bytes) - returns true if all bytes are non-zero
// Empty bytes returns True for all()
int64_t bytes_all(const char* s) {
    if (s == NULL) return 1;
    size_t len = strlen(s);
    if (len == 0) return 1;
    for (size_t i = 0; i < len; i++) {
        if ((unsigned char)s[i] == 0) return 0;
    }
    return 1;
}

// repr(bytes) - returns string like "b'hello'"
char* repr_bytes(const char* s) {
    if (s == NULL) return strdup("b''");
    size_t len = strlen(s);
    size_t result_len = len + 4; // b'' + null
    char* result = (char*)malloc(result_len);
    if (result == NULL) return NULL;
    snprintf(result, result_len, "b'%s'", s);
    return result;
}
