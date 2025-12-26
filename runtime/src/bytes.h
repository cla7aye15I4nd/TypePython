#ifndef BYTES_H
#define BYTES_H

#include "types.h"
#include "str.h"

// ============================================================================
// Bytes structure with flexible array member
// Immutable bytes type (similar to Python's bytes)
// ============================================================================

typedef struct {
    int64_t len;        // Length of the bytes sequence
    uint8_t data[];     // Flexible array member for byte data
} Bytes;

// Bytes operations
Bytes* BYTES_METHOD(__init__)(const uint8_t* data, int64_t len);
void BYTES_METHOD(free)(Bytes* b);
int64_t BYTES_METHOD(__len__)(Bytes* b);
int64_t BYTES_METHOD(__getitem__)(Bytes* b, int64_t index);

// bytes.__str__() and bytes.__repr__() - both return b'...' format
String* BYTES_METHOD(__str__)(Bytes* b);
String* BYTES_METHOD(__repr__)(Bytes* b);

#endif // BYTES_H
