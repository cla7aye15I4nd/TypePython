// TypePython Runtime - Common Utilities
// Shared helper functions used across multiple modules

#ifndef TPY_COMMON_H
#define TPY_COMMON_H

#include <stdint.h>

// Normalize negative index to positive (Python-style indexing)
static inline int64_t tpy_normalize_index(int64_t index, int64_t len) {
    if (index < 0) index = len + index;
    return index;
}

// Hash function for int64 keys (used by dict and set)
static inline uint64_t tpy_hash_int(int64_t key) {
    uint64_t h = (uint64_t)key;
    h ^= h >> 33;
    h *= 0xff51afd7ed558ccdULL;
    h ^= h >> 33;
    h *= 0xc4ceb9fe1a85ec53ULL;
    h ^= h >> 33;
    return h;
}

// Hash function for strings (FNV-1a)
static inline uint64_t tpy_hash_str(const char* s) {
    uint64_t hash = 14695981039346656037ULL;
    while (*s) {
        hash ^= (unsigned char)*s++;
        hash *= 1099511628211ULL;
    }
    return hash;
}

#endif // TPY_COMMON_H
