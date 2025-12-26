#include "bytes.h"
#include <stdlib.h>
#include <string.h>

// ============================================================================
// Bytes operations
// ============================================================================

Bytes* BYTES_METHOD(__init__)(const uint8_t* data, int64_t len) {
    if (len < 0) return NULL;

    Bytes* b = (Bytes*)malloc(sizeof(Bytes) + len);
    if (b == NULL) return NULL;

    b->len = len;
    if (data != NULL && len > 0) {
        memcpy(b->data, data, len);
    }
    return b;
}

void BYTES_METHOD(free)(Bytes* b) {
    free(b);
}

int64_t BYTES_METHOD(__len__)(Bytes* b) {
    return b ? b->len : 0;
}

int64_t BYTES_METHOD(__getitem__)(Bytes* b, int64_t index) {
    if (b == NULL || index < 0 || index >= b->len) {
        return -1;
    }
    return (int64_t)b->data[index];
}

String* BYTES_METHOD(__repr__)(Bytes* b) {
    if (b == NULL) {
        return STR_METHOD(from_literal)("b''", 3);
    }

    // Calculate output length
    int64_t out_len = 3;  // b' and '
    for (int64_t i = 0; i < b->len; i++) {
        uint8_t c = b->data[i];
        if (c == '\\' || c == '\'') {
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
    result->data[pos++] = 'b';
    result->data[pos++] = '\'';

    for (int64_t i = 0; i < b->len; i++) {
        uint8_t c = b->data[i];
        if (c == '\\') {
            result->data[pos++] = '\\'; result->data[pos++] = '\\';
        } else if (c == '\'') {
            result->data[pos++] = '\\'; result->data[pos++] = '\'';
        } else if (c >= 32 && c < 127) {
            result->data[pos++] = (char)c;
        } else {
            result->data[pos++] = '\\';
            result->data[pos++] = 'x';
            result->data[pos++] = "0123456789abcdef"[c >> 4];
            result->data[pos++] = "0123456789abcdef"[c & 0xf];
        }
    }

    result->data[pos++] = '\'';
    result->data[pos] = '\0';
    return result;
}

String* BYTES_METHOD(__str__)(Bytes* b) {
    return BYTES_METHOD(__repr__)(b);
}
