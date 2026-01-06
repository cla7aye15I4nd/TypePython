#ifndef STR_H
#define STR_H

#include "types.h"

#ifndef NO_ICU
#include <unicode/ubrk.h>
#include <unicode/utf8.h>
#include <unicode/ustring.h>
#include <unicode/ucasemap.h>
#include <unicode/unorm2.h>
#endif

// ============================================================================
// String structure with flexible array member
// Immutable string type (similar to Python's str)
// ============================================================================

// String flags
#define STR_FLAG_ASCII_ONLY  0x01  // All characters are ASCII (0-127)
#define STR_FLAG_VALID_UTF8  0x02  // String is valid UTF-8

typedef struct {
    int64_t len;             // Byte length (excluding null terminator)
    int32_t cp_count;        // Cached Unicode codepoint count (-1 = not computed)
    uint16_t flags;          // STR_FLAG_ASCII_ONLY | STR_FLAG_VALID_UTF8
    char data[];             // UTF-8 encoded data
} String;

// String creation
String* STR_METHOD(__init__)(const char* cstr);
String* STR_METHOD(from_literal)(const char* cstr, int64_t len);
void STR_METHOD(free)(String* s);

// String operations - use String* struct
int64_t STR_METHOD(__len__)(String* str);
int64_t STR_METHOD(__getitem__)(String* s, int64_t index);  // Returns Unicode codepoint

// String concatenation
String* STR_METHOD(__add__)(String* a, String* b);

// String comparison operators
int8_t STR_METHOD(__eq__)(String* a, String* b);
int8_t STR_METHOD(__ne__)(String* a, String* b);
int8_t STR_METHOD(__lt__)(String* a, String* b);
int8_t STR_METHOD(__le__)(String* a, String* b);
int8_t STR_METHOD(__gt__)(String* a, String* b);
int8_t STR_METHOD(__ge__)(String* a, String* b);

// str.__str__() returns self (identity function - returns data pointer)
String* STR_METHOD(__str__)(String* str);

// str.__repr__() returns quoted string with escape sequences
String* STR_METHOD(__repr__)(String* str);

// Helper to get raw char* data from String* (for C interop)
static inline const char* string_data(String* s) {
    return s ? s->data : "";
}

// Helper to get length from String*
static inline int64_t string_len(String* s) {
    return s ? s->len : 0;
}

// ============================================================================
// Phase 3: Common String Methods
// ============================================================================

// Case conversion
String* STR_METHOD(lower)(String* str);
String* STR_METHOD(upper)(String* str);

// Whitespace operations
String* STR_METHOD(strip)(String* str);

// String search
int64_t STR_METHOD(find)(String* str, String* substr);
int8_t STR_METHOD(startswith)(String* str, String* prefix);
int8_t STR_METHOD(endswith)(String* str, String* suffix);

// String modification
String* STR_METHOD(replace)(String* str, String* old, String* new_str);

// Character classification
int8_t STR_METHOD(isalpha)(String* str);
int8_t STR_METHOD(isdigit)(String* str);
int8_t STR_METHOD(isspace)(String* str);

// ============================================================================
// Phase 5: String Normalization
// ============================================================================

// Unicode normalization forms
String* STR_METHOD(normalize_nfc)(String* str);
String* STR_METHOD(normalize_nfd)(String* str);
String* STR_METHOD(normalize_nfkc)(String* str);
String* STR_METHOD(normalize_nfkd)(String* str);

#endif // STR_H
