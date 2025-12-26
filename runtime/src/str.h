#ifndef STR_H
#define STR_H

#include "types.h"

// ============================================================================
// String structure with flexible array member
// Immutable string type (similar to Python's str)
// ============================================================================

typedef struct {
    int64_t len;        // Length of the string (excluding null terminator)
    char data[];        // Flexible array member for string data
} String;

// String creation
String* STR_METHOD(__init__)(const char* cstr);
String* STR_METHOD(from_literal)(const char* cstr, int64_t len);
void STR_METHOD(free)(String* s);

// String operations - use String* struct
int64_t STR_METHOD(__len__)(String* str);
char STR_METHOD(__getitem__)(String* s, int64_t index);

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

#endif // STR_H
