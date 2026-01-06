#include "str.h"
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

// ============================================================================
// String operations with Unicode support (ICU)
// ============================================================================

// Helper: Check if a string is ASCII-only
static inline uint16_t detect_flags(const char* data, int64_t len) {
    uint16_t flags = STR_FLAG_VALID_UTF8;  // Assume valid UTF-8
    int8_t is_ascii = 1;

    for (int64_t i = 0; i < len; i++) {
        if ((unsigned char)data[i] > 127) {
            is_ascii = 0;
            break;
        }
    }

    if (is_ascii) {
        flags |= STR_FLAG_ASCII_ONLY;
    }

    return flags;
}

String* STR_METHOD(__init__)(const char* cstr) {
    if (cstr == NULL) {
        String* s = (String*)malloc(sizeof(String) + 1);
        if (s == NULL) return NULL;
        s->len = 0;
        s->cp_count = 0;
        s->flags = STR_FLAG_ASCII_ONLY | STR_FLAG_VALID_UTF8;
        s->data[0] = '\0';
        return s;
    }

    size_t len = strlen(cstr);
    String* s = (String*)malloc(sizeof(String) + len + 1);
    if (s == NULL) return NULL;

    s->len = (int64_t)len;
    s->flags = detect_flags(cstr, len);
    s->cp_count = -1;  // Not computed yet
    memcpy(s->data, cstr, len + 1);  // Include null terminator
    return s;
}

String* STR_METHOD(from_literal)(const char* cstr, int64_t len) {
    String* s = (String*)malloc(sizeof(String) + len + 1);
    if (s == NULL) return NULL;

    s->len = len;
    s->flags = detect_flags(cstr, len);
    s->cp_count = -1;  // Not computed yet
    memcpy(s->data, cstr, len);
    s->data[len] = '\0';
    return s;
}

void STR_METHOD(free)(String* s) {
    free(s);
}

int64_t STR_METHOD(__len__)(String* str) {
    if (!str) return 0;

    // If already computed, return cached value
    if (str->cp_count >= 0) {
        return str->cp_count;
    }

    // Fast path for ASCII strings
    if (str->flags & STR_FLAG_ASCII_ONLY) {
        // For ASCII strings, byte length == character count
        // Note: Don't cache for literals (they're read-only)
        return str->len;
    }

#ifdef NO_ICU
    // Without ICU, count UTF-8 codepoints manually
    int32_t count = 0;
    int32_t byte_idx = 0;
    while (byte_idx < str->len) {
        unsigned char c = (unsigned char)str->data[byte_idx];
        if ((c & 0x80) == 0) byte_idx += 1;           // 0xxxxxxx - ASCII
        else if ((c & 0xE0) == 0xC0) byte_idx += 2;   // 110xxxxx
        else if ((c & 0xF0) == 0xE0) byte_idx += 3;   // 1110xxxx
        else if ((c & 0xF8) == 0xF0) byte_idx += 4;   // 11110xxx
        else byte_idx += 1;  // Invalid, skip
        count++;
    }
    return count;
#else
    // Count Unicode codepoints to match Python's len() behavior
    // Note: Python counts codepoints, not grapheme clusters
    // So len("👋🏽") == 2 in Python (two codepoints: wave + skin tone modifier)
    int32_t count = 0;
    int32_t byte_idx = 0;
    while (byte_idx < str->len) {
        UChar32 c;
        U8_NEXT(str->data, byte_idx, str->len, c);
        if (c >= 0) count++;
    }
    return count;
#endif
}

int64_t STR_METHOD(__getitem__)(String* s, int64_t index) {
    if (s == NULL || index < 0) {
        return -1;  // Error: invalid input
    }

    // Fast path for ASCII strings
    if (s->flags & STR_FLAG_ASCII_ONLY) {
        if (index >= s->len) {
            return -1;  // Out of bounds
        }
        return (int64_t)(unsigned char)s->data[index];
    }

#ifdef NO_ICU
    // Without ICU, manually decode UTF-8 codepoints
    int32_t byte_idx = 0;
    int64_t current_idx = 0;

    while (byte_idx < s->len) {
        unsigned char c = (unsigned char)s->data[byte_idx];
        int32_t codepoint = 0;
        int bytes = 0;

        if ((c & 0x80) == 0) {
            codepoint = c;
            bytes = 1;
        } else if ((c & 0xE0) == 0xC0) {
            codepoint = c & 0x1F;
            bytes = 2;
        } else if ((c & 0xF0) == 0xE0) {
            codepoint = c & 0x0F;
            bytes = 3;
        } else if ((c & 0xF8) == 0xF0) {
            codepoint = c & 0x07;
            bytes = 4;
        } else {
            return -1;  // Invalid UTF-8
        }

        for (int i = 1; i < bytes && (byte_idx + i) < s->len; i++) {
            codepoint = (codepoint << 6) | (s->data[byte_idx + i] & 0x3F);
        }

        if (current_idx == index) {
            return (int64_t)codepoint;
        }
        byte_idx += bytes;
        current_idx++;
    }

    return -1;  // Index out of bounds
#else
    // For Unicode strings, navigate to the index-th codepoint
    int32_t byte_idx = 0;
    UChar32 codepoint = 0;
    int64_t current_idx = 0;

    while (byte_idx < s->len) {
        U8_NEXT(s->data, byte_idx, s->len, codepoint);
        if (codepoint < 0) {
            return -1;  // Invalid UTF-8
        }
        if (current_idx == index) {
            return (int64_t)codepoint;
        }
        current_idx++;
    }

    return -1;  // Index out of bounds
#endif
}

String* STR_METHOD(__str__)(String* str) {
    return str;
}

String* STR_METHOD(__repr__)(String* str) {
    if (str == NULL) {
        return STR_METHOD(from_literal)("''", 2);
    }

    // Calculate output length
    int64_t out_len = 2;  // Quotes
    for (int64_t i = 0; i < str->len; i++) {
        char c = str->data[i];
        if (c == '\n' || c == '\t' || c == '\r' || c == '\\' || c == '\'') {
            out_len += 2;
        } else if (c >= 32 && c < 127) {
            out_len += 1;
        } else {
            out_len += 4;  // \xNN
        }
    }

    String* result = (String*)malloc(sizeof(String) + out_len + 1);
    if (result == NULL) return NULL;
    result->len = out_len;
    result->cp_count = -1;  // Not computed
    result->flags = STR_FLAG_ASCII_ONLY | STR_FLAG_VALID_UTF8;  // __repr__ output is ASCII

    int64_t pos = 0;
    result->data[pos++] = '\'';

    for (int64_t i = 0; i < str->len; i++) {
        char c = str->data[i];
        if (c == '\n') {
            result->data[pos++] = '\\'; result->data[pos++] = 'n';
        } else if (c == '\t') {
            result->data[pos++] = '\\'; result->data[pos++] = 't';
        } else if (c == '\r') {
            result->data[pos++] = '\\'; result->data[pos++] = 'r';
        } else if (c == '\\') {
            result->data[pos++] = '\\'; result->data[pos++] = '\\';
        } else if (c == '\'') {
            result->data[pos++] = '\\'; result->data[pos++] = '\'';
        } else if (c >= 32 && c < 127) {
            result->data[pos++] = c;
        } else {
            result->data[pos++] = '\\';
            result->data[pos++] = 'x';
            result->data[pos++] = "0123456789abcdef"[(unsigned char)c >> 4];
            result->data[pos++] = "0123456789abcdef"[(unsigned char)c & 0xf];
        }
    }

    result->data[pos++] = '\'';
    result->data[pos] = '\0';
    return result;
}

// ============================================================================
// String concatenation (CRITICAL - was broken before)
// ============================================================================

String* STR_METHOD(__add__)(String* a, String* b) {
    if (a == NULL && b == NULL) {
        return STR_METHOD(__init__)(NULL);  // Empty string
    }
    if (a == NULL) return b;
    if (b == NULL) return a;

    int64_t total_len = a->len + b->len;
    String* result = (String*)malloc(sizeof(String) + total_len + 1);
    if (result == NULL) return NULL;

    result->len = total_len;
    result->cp_count = -1;  // Will be computed on demand

    // Copy both strings
    memcpy(result->data, a->data, a->len);
    memcpy(result->data + a->len, b->data, b->len);
    result->data[total_len] = '\0';

    // Set flags: ASCII only if both are ASCII
    if ((a->flags & STR_FLAG_ASCII_ONLY) && (b->flags & STR_FLAG_ASCII_ONLY)) {
        result->flags = STR_FLAG_ASCII_ONLY | STR_FLAG_VALID_UTF8;
        result->cp_count = (int32_t)total_len;  // ASCII: byte count == char count
    } else {
        result->flags = STR_FLAG_VALID_UTF8;
    }

    return result;
}

// ============================================================================
// String comparison operators
// ============================================================================

int8_t STR_METHOD(__eq__)(String* a, String* b) {
    if (a == b) return 1;  // Same pointer
    if (a == NULL || b == NULL) return 0;
    if (a->len != b->len) return 0;
    return memcmp(a->data, b->data, a->len) == 0 ? 1 : 0;
}

int8_t STR_METHOD(__ne__)(String* a, String* b) {
    return !STR_METHOD(__eq__)(a, b);
}

int8_t STR_METHOD(__lt__)(String* a, String* b) {
    if (a == NULL || b == NULL) return 0;
    int64_t min_len = a->len < b->len ? a->len : b->len;
    int cmp = memcmp(a->data, b->data, min_len);
    if (cmp < 0) return 1;
    if (cmp > 0) return 0;
    return a->len < b->len ? 1 : 0;
}

int8_t STR_METHOD(__le__)(String* a, String* b) {
    return STR_METHOD(__eq__)(a, b) || STR_METHOD(__lt__)(a, b);
}

int8_t STR_METHOD(__gt__)(String* a, String* b) {
    if (a == NULL || b == NULL) return 0;
    int64_t min_len = a->len < b->len ? a->len : b->len;
    int cmp = memcmp(a->data, b->data, min_len);
    if (cmp > 0) return 1;
    if (cmp < 0) return 0;
    return a->len > b->len ? 1 : 0;
}

int8_t STR_METHOD(__ge__)(String* a, String* b) {
    return STR_METHOD(__eq__)(a, b) || STR_METHOD(__gt__)(a, b);
}

// ============================================================================
// Phase 3: Common String Methods
// ============================================================================

// Case conversion methods
String* STR_METHOD(lower)(String* str) {
    if (str == NULL) return NULL;

    // Fast path for ASCII strings
    if (str->flags & STR_FLAG_ASCII_ONLY) {
        String* result = (String*)malloc(sizeof(String) + str->len + 1);
        if (result == NULL) return NULL;

        result->len = str->len;
        result->cp_count = str->cp_count;
        result->flags = str->flags;

        for (int64_t i = 0; i < str->len; i++) {
            char c = str->data[i];
            result->data[i] = (c >= 'A' && c <= 'Z') ? (c + 32) : c;
        }
        result->data[str->len] = '\0';
        return result;
    }

#ifdef NO_ICU
    // Without ICU, only handle ASCII (already done above), return copy for non-ASCII
    String* result = (String*)malloc(sizeof(String) + str->len + 1);
    if (result == NULL) return NULL;
    result->len = str->len;
    result->cp_count = str->cp_count;
    result->flags = str->flags;
    memcpy(result->data, str->data, str->len + 1);
    return result;
#else
    // Unicode case conversion using ICU
    UErrorCode status = U_ZERO_ERROR;
    UCaseMap* csm = ucasemap_open("en_US", 0, &status);
    if (U_FAILURE(status)) return NULL;

    // Calculate required buffer size
    int32_t dest_len = ucasemap_utf8ToLower(csm, NULL, 0,
        str->data, str->len, &status);

    if (status != U_BUFFER_OVERFLOW_ERROR && U_FAILURE(status)) {
        ucasemap_close(csm);
        return NULL;
    }

    // Allocate and convert
    String* result = (String*)malloc(sizeof(String) + dest_len + 1);
    if (result == NULL) {
        ucasemap_close(csm);
        return NULL;
    }

    status = U_ZERO_ERROR;
    dest_len = ucasemap_utf8ToLower(csm, result->data, dest_len + 1,
        str->data, str->len, &status);

    ucasemap_close(csm);

    if (U_FAILURE(status)) {
        free(result);
        return NULL;
    }

    result->len = dest_len;
    result->cp_count = -1;  // Not computed
    result->flags = STR_FLAG_VALID_UTF8;
    result->data[dest_len] = '\0';

    return result;
#endif
}

String* STR_METHOD(upper)(String* str) {
    if (str == NULL) return NULL;

    // Fast path for ASCII strings
    if (str->flags & STR_FLAG_ASCII_ONLY) {
        String* result = (String*)malloc(sizeof(String) + str->len + 1);
        if (result == NULL) return NULL;

        result->len = str->len;
        result->cp_count = str->cp_count;
        result->flags = str->flags;

        for (int64_t i = 0; i < str->len; i++) {
            char c = str->data[i];
            result->data[i] = (c >= 'a' && c <= 'z') ? (c - 32) : c;
        }
        result->data[str->len] = '\0';
        return result;
    }

#ifdef NO_ICU
    // Without ICU, only handle ASCII (already done above), return copy for non-ASCII
    String* result = (String*)malloc(sizeof(String) + str->len + 1);
    if (result == NULL) return NULL;
    result->len = str->len;
    result->cp_count = str->cp_count;
    result->flags = str->flags;
    memcpy(result->data, str->data, str->len + 1);
    return result;
#else
    // Unicode case conversion using ICU
    UErrorCode status = U_ZERO_ERROR;
    UCaseMap* csm = ucasemap_open("en_US", 0, &status);
    if (U_FAILURE(status)) return NULL;

    // Calculate required buffer size
    int32_t dest_len = ucasemap_utf8ToUpper(csm, NULL, 0,
        str->data, str->len, &status);

    if (status != U_BUFFER_OVERFLOW_ERROR && U_FAILURE(status)) {
        ucasemap_close(csm);
        return NULL;
    }

    // Allocate and convert
    String* result = (String*)malloc(sizeof(String) + dest_len + 1);
    if (result == NULL) {
        ucasemap_close(csm);
        return NULL;
    }

    status = U_ZERO_ERROR;
    dest_len = ucasemap_utf8ToUpper(csm, result->data, dest_len + 1,
        str->data, str->len, &status);

    ucasemap_close(csm);

    if (U_FAILURE(status)) {
        free(result);
        return NULL;
    }

    result->len = dest_len;
    result->cp_count = -1;  // Not computed
    result->flags = STR_FLAG_VALID_UTF8;
    result->data[dest_len] = '\0';

    return result;
#endif
}

// Whitespace operations
String* STR_METHOD(strip)(String* str) {
    if (str == NULL) return NULL;
    if (str->len == 0) return str;

    // Find first non-whitespace character
    int64_t start = 0;
    while (start < str->len && (str->data[start] == ' ' || str->data[start] == '\t' ||
                                 str->data[start] == '\n' || str->data[start] == '\r')) {
        start++;
    }

    // Find last non-whitespace character
    int64_t end = str->len - 1;
    while (end >= start && (str->data[end] == ' ' || str->data[end] == '\t' ||
                            str->data[end] == '\n' || str->data[end] == '\r')) {
        end--;
    }

    if (start > end) {
        // All whitespace - return empty string
        return STR_METHOD(from_literal)("", 0);
    }

    int64_t new_len = end - start + 1;
    if (new_len == str->len) {
        // No whitespace to strip - return original
        return str;
    }

    String* result = (String*)malloc(sizeof(String) + new_len + 1);
    if (result == NULL) return NULL;

    result->len = new_len;
    result->cp_count = -1;
    result->flags = str->flags;

    memcpy(result->data, str->data + start, new_len);
    result->data[new_len] = '\0';

    return result;
}

// String search methods
int64_t STR_METHOD(find)(String* str, String* substr) {
    if (str == NULL || substr == NULL) return -1;
    if (substr->len == 0) return 0;
    if (substr->len > str->len) return -1;

    // Simple Boyer-Moore-like search
    for (int64_t i = 0; i <= str->len - substr->len; i++) {
        if (memcmp(str->data + i, substr->data, substr->len) == 0) {
            return i;
        }
    }

    return -1;
}

int8_t STR_METHOD(startswith)(String* str, String* prefix) {
    if (str == NULL || prefix == NULL) return 0;
    if (prefix->len > str->len) return 0;
    if (prefix->len == 0) return 1;

    return memcmp(str->data, prefix->data, prefix->len) == 0 ? 1 : 0;
}

int8_t STR_METHOD(endswith)(String* str, String* suffix) {
    if (str == NULL || suffix == NULL) return 0;
    if (suffix->len > str->len) return 0;
    if (suffix->len == 0) return 1;

    return memcmp(str->data + (str->len - suffix->len),
                  suffix->data, suffix->len) == 0 ? 1 : 0;
}

// String modification
String* STR_METHOD(replace)(String* str, String* old, String* new_str) {
    if (str == NULL || old == NULL || new_str == NULL) return str;
    if (old->len == 0) return str;  // Can't replace empty string

    // Count occurrences
    int64_t count = 0;
    for (int64_t i = 0; i <= str->len - old->len; i++) {
        if (memcmp(str->data + i, old->data, old->len) == 0) {
            count++;
            i += old->len - 1;  // Skip past this occurrence
        }
    }

    if (count == 0) return str;  // No replacements needed

    // Calculate new length
    int64_t new_len = str->len + count * (new_str->len - old->len);

    String* result = (String*)malloc(sizeof(String) + new_len + 1);
    if (result == NULL) return NULL;

    result->len = new_len;
    result->cp_count = -1;
    result->flags = (str->flags & new_str->flags);  // ASCII only if both are ASCII

    // Perform replacements
    int64_t src_pos = 0;
    int64_t dst_pos = 0;

    while (src_pos < str->len) {
        if (src_pos <= str->len - old->len &&
            memcmp(str->data + src_pos, old->data, old->len) == 0) {
            // Found match - copy replacement
            memcpy(result->data + dst_pos, new_str->data, new_str->len);
            dst_pos += new_str->len;
            src_pos += old->len;
        } else {
            // No match - copy character
            result->data[dst_pos++] = str->data[src_pos++];
        }
    }

    result->data[new_len] = '\0';
    return result;
}

// Character classification methods
int8_t STR_METHOD(isalpha)(String* str) {
    if (str == NULL || str->len == 0) return 0;

#ifdef NO_ICU
    // ASCII-only fallback
    for (int64_t i = 0; i < str->len; i++) {
        char c = str->data[i];
        if (!((c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z'))) {
            return 0;
        }
    }
    return 1;
#else
    for (int64_t i = 0; i < str->len; ) {
        UChar32 c;
        U8_NEXT(str->data, i, str->len, c);
        if (c < 0 || !u_isalpha(c)) {
            return 0;
        }
    }

    return 1;
#endif
}

int8_t STR_METHOD(isdigit)(String* str) {
    if (str == NULL || str->len == 0) return 0;

#ifdef NO_ICU
    // ASCII-only fallback
    for (int64_t i = 0; i < str->len; i++) {
        char c = str->data[i];
        if (c < '0' || c > '9') {
            return 0;
        }
    }
    return 1;
#else
    for (int64_t i = 0; i < str->len; ) {
        UChar32 c;
        U8_NEXT(str->data, i, str->len, c);
        if (c < 0 || !u_isdigit(c)) {
            return 0;
        }
    }

    return 1;
#endif
}

int8_t STR_METHOD(isspace)(String* str) {
    if (str == NULL || str->len == 0) return 0;

#ifdef NO_ICU
    // ASCII-only fallback
    for (int64_t i = 0; i < str->len; i++) {
        char c = str->data[i];
        if (c != ' ' && c != '\t' && c != '\n' && c != '\r' && c != '\f' && c != '\v') {
            return 0;
        }
    }
    return 1;
#else
    for (int64_t i = 0; i < str->len; ) {
        UChar32 c;
        U8_NEXT(str->data, i, str->len, c);
        if (c < 0 || !u_isspace(c)) {
            return 0;
        }
    }

    return 1;
#endif
}
