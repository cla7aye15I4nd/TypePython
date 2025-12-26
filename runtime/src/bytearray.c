#include "runtime.h"
#include <stdlib.h>
#include <string.h>

ByteArray* BYTEARRAY_METHOD(__init__)(void) {
    ByteArray* ba = (ByteArray*)malloc(sizeof(ByteArray));
    if (ba == NULL) {
        rt_panic("Failed to allocate memory for bytearray");
    }

    ba->cap = 8;
    ba->len = 0;
    ba->data = (uint8_t*)malloc(ba->cap);

    if (ba->data == NULL) {
        rt_panic("Failed to allocate memory for bytearray data");
    }

    return ba;
}

void BYTEARRAY_METHOD(append)(ByteArray* ba, int64_t value) {
    if (ba == NULL) {
        rt_panic("Cannot append to NULL bytearray");
    }
    if (value < 0 || value > 255) {
        rt_panic("bytearray value out of range (0-255)");
    }

    if (ba->len == ba->cap) {
        ba->cap *= 2;
        uint8_t* new_data = (uint8_t*)realloc(ba->data, ba->cap);
        if (new_data == NULL) {
            rt_panic("Failed to reallocate memory for bytearray");
        }
        ba->data = new_data;
    }

    ba->data[ba->len++] = (uint8_t)value;
}

int64_t BYTEARRAY_METHOD(__getitem__)(ByteArray* ba, int64_t index) {
    if (ba == NULL) {
        rt_panic("Cannot get from NULL bytearray");
    }
    if (index < 0 || index >= ba->len) {
        rt_panic_index("Index out of bounds", index, ba->len);
    }
    return (int64_t)ba->data[index];
}

void BYTEARRAY_METHOD(__setitem__)(ByteArray* ba, int64_t index, int64_t value) {
    if (ba == NULL) {
        rt_panic("Cannot set in NULL bytearray");
    }
    if (index < 0 || index >= ba->len) {
        rt_panic_index("Index out of bounds", index, ba->len);
    }
    if (value < 0 || value > 255) {
        rt_panic("bytearray value out of range (0-255)");
    }
    ba->data[index] = (uint8_t)value;
}

int64_t BYTEARRAY_METHOD(__len__)(ByteArray* ba) {
    if (ba == NULL) {
        rt_panic("Cannot get length of NULL bytearray");
    }
    return ba->len;
}

void BYTEARRAY_METHOD(free)(ByteArray* ba) {
    if (ba != NULL) {
        free(ba->data);
        free(ba);
    }
}

String* BYTEARRAY_METHOD(__repr__)(ByteArray* ba) {
    if (ba == NULL) {
        return STR_METHOD(from_literal)("bytearray(b'')", 14);
    }

    // Calculate output length: "bytearray(b'...')"
    int64_t out_len = 14;  // bytearray(b'')
    for (int64_t i = 0; i < ba->len; i++) {
        uint8_t c = ba->data[i];
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

    memcpy(result->data, "bytearray(b'", 12);
    pos = 12;

    for (int64_t i = 0; i < ba->len; i++) {
        uint8_t c = ba->data[i];
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
    result->data[pos++] = ')';
    result->data[pos] = '\0';

    return result;
}

String* BYTEARRAY_METHOD(__str__)(ByteArray* ba) {
    return BYTEARRAY_METHOD(__repr__)(ba);
}
