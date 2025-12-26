#include "str.h"
#include <stdlib.h>
#include <string.h>

// ============================================================================
// String operations
// ============================================================================

String* STR_METHOD(__init__)(const char* cstr) {
    if (cstr == NULL) {
        String* s = (String*)malloc(sizeof(String) + 1);
        if (s == NULL) return NULL;
        s->len = 0;
        s->data[0] = '\0';
        return s;
    }

    size_t len = strlen(cstr);
    String* s = (String*)malloc(sizeof(String) + len + 1);
    if (s == NULL) return NULL;

    s->len = (int64_t)len;
    memcpy(s->data, cstr, len + 1);  // Include null terminator
    return s;
}

String* STR_METHOD(from_literal)(const char* cstr, int64_t len) {
    String* s = (String*)malloc(sizeof(String) + len + 1);
    if (s == NULL) return NULL;

    s->len = len;
    memcpy(s->data, cstr, len);
    s->data[len] = '\0';
    return s;
}

void STR_METHOD(free)(String* s) {
    free(s);
}

int64_t STR_METHOD(__len__)(String* str) {
    return str ? str->len : 0;
}

char STR_METHOD(__getitem__)(String* s, int64_t index) {
    if (s == NULL || index < 0 || index >= s->len) {
        return '\0';
    }
    return s->data[index];
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
