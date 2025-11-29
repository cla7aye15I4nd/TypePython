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

// ============================================================================
// SDS Type Definitions and Macros
// ============================================================================

typedef char *sds;

#define SDS_MAX_PREALLOC (1024*1024)

// SDS header types - allows different header sizes for different string lengths
#define SDS_TYPE_5  0
#define SDS_TYPE_8  1
#define SDS_TYPE_16 2
#define SDS_TYPE_32 3
#define SDS_TYPE_64 4
#define SDS_TYPE_MASK 7
#define SDS_TYPE_BITS 3

// SDS headers for different sizes
struct __attribute__ ((__packed__)) sdshdr5 {
    unsigned char flags; // 3 lsb of type, and 5 msb of string length
    char buf[];
};

struct __attribute__ ((__packed__)) sdshdr8 {
    uint8_t len;
    uint8_t alloc;
    unsigned char flags;
    char buf[];
};

struct __attribute__ ((__packed__)) sdshdr16 {
    uint16_t len;
    uint16_t alloc;
    unsigned char flags;
    char buf[];
};

struct __attribute__ ((__packed__)) sdshdr32 {
    uint32_t len;
    uint32_t alloc;
    unsigned char flags;
    char buf[];
};

struct __attribute__ ((__packed__)) sdshdr64 {
    uint64_t len;
    uint64_t alloc;
    unsigned char flags;
    char buf[];
};

#define SDS_TYPE_5_LEN(f) ((f)>>SDS_TYPE_BITS)

// Macros to access SDS header
#define SDS_HDR_VAR(T,s) struct sdshdr##T *sh = (void*)((s)-(sizeof(struct sdshdr##T)));
#define SDS_HDR(T,s) ((struct sdshdr##T *)((s)-(sizeof(struct sdshdr##T))))

// ============================================================================
// SDS Core Functions (static - internal use only)
// ============================================================================

static inline int sdsHdrSize(char type) {
    switch(type&SDS_TYPE_MASK) {
        case SDS_TYPE_5:  return sizeof(struct sdshdr5);
        case SDS_TYPE_8:  return sizeof(struct sdshdr8);
        case SDS_TYPE_16: return sizeof(struct sdshdr16);
        case SDS_TYPE_32: return sizeof(struct sdshdr32);
        case SDS_TYPE_64: return sizeof(struct sdshdr64);
    }
    return 0;
}

static inline char sdsReqType(size_t string_size) {
    if (string_size < 1<<5)
        return SDS_TYPE_5;
    if (string_size < 1<<8)
        return SDS_TYPE_8;
    if (string_size < 1<<16)
        return SDS_TYPE_16;
#if (LONG_MAX == LLONG_MAX)
    if (string_size < 1ll<<32)
        return SDS_TYPE_32;
    return SDS_TYPE_64;
#else
    return SDS_TYPE_32;
#endif
}

static inline size_t sdslen(const sds s) {
    unsigned char flags = s[-1];
    switch(flags&SDS_TYPE_MASK) {
        case SDS_TYPE_5:
            return SDS_TYPE_5_LEN(flags);
        case SDS_TYPE_8:
            return SDS_HDR(8,s)->len;
        case SDS_TYPE_16:
            return SDS_HDR(16,s)->len;
        case SDS_TYPE_32:
            return SDS_HDR(32,s)->len;
        case SDS_TYPE_64:
            return SDS_HDR(64,s)->len;
    }
    return 0;
}

static inline size_t sdsavail(const sds s) {
    unsigned char flags = s[-1];
    switch(flags&SDS_TYPE_MASK) {
        case SDS_TYPE_5:
            return 0;
        case SDS_TYPE_8: {
            SDS_HDR_VAR(8,s);
            return sh->alloc - sh->len;
        }
        case SDS_TYPE_16: {
            SDS_HDR_VAR(16,s);
            return sh->alloc - sh->len;
        }
        case SDS_TYPE_32: {
            SDS_HDR_VAR(32,s);
            return sh->alloc - sh->len;
        }
        case SDS_TYPE_64: {
            SDS_HDR_VAR(64,s);
            return sh->alloc - sh->len;
        }
    }
    return 0;
}

static inline void sdssetlen(sds s, size_t newlen) {
    unsigned char flags = s[-1];
    switch(flags&SDS_TYPE_MASK) {
        case SDS_TYPE_5:
            {
                unsigned char *fp = ((unsigned char*)s)-1;
                *fp = SDS_TYPE_5 | (newlen << SDS_TYPE_BITS);
            }
            break;
        case SDS_TYPE_8:
            SDS_HDR(8,s)->len = newlen;
            break;
        case SDS_TYPE_16:
            SDS_HDR(16,s)->len = newlen;
            break;
        case SDS_TYPE_32:
            SDS_HDR(32,s)->len = newlen;
            break;
        case SDS_TYPE_64:
            SDS_HDR(64,s)->len = newlen;
            break;
    }
}

static inline void sdsinclen(sds s, size_t inc) {
    unsigned char flags = s[-1];
    switch(flags&SDS_TYPE_MASK) {
        case SDS_TYPE_5:
            {
                unsigned char *fp = ((unsigned char*)s)-1;
                unsigned char newlen = SDS_TYPE_5_LEN(flags)+inc;
                *fp = SDS_TYPE_5 | (newlen << SDS_TYPE_BITS);
            }
            break;
        case SDS_TYPE_8:
            SDS_HDR(8,s)->len += inc;
            break;
        case SDS_TYPE_16:
            SDS_HDR(16,s)->len += inc;
            break;
        case SDS_TYPE_32:
            SDS_HDR(32,s)->len += inc;
            break;
        case SDS_TYPE_64:
            SDS_HDR(64,s)->len += inc;
            break;
    }
}

static inline size_t sdsalloc(const sds s) {
    unsigned char flags = s[-1];
    switch(flags&SDS_TYPE_MASK) {
        case SDS_TYPE_5:
            return SDS_TYPE_5_LEN(flags);
        case SDS_TYPE_8:
            return SDS_HDR(8,s)->alloc;
        case SDS_TYPE_16:
            return SDS_HDR(16,s)->alloc;
        case SDS_TYPE_32:
            return SDS_HDR(32,s)->alloc;
        case SDS_TYPE_64:
            return SDS_HDR(64,s)->alloc;
    }
    return 0;
}

static inline void sdssetalloc(sds s, size_t newlen) {
    unsigned char flags = s[-1];
    switch(flags&SDS_TYPE_MASK) {
        case SDS_TYPE_5:
            break; // Nothing to do, type 5 has no alloc field
        case SDS_TYPE_8:
            SDS_HDR(8,s)->alloc = newlen;
            break;
        case SDS_TYPE_16:
            SDS_HDR(16,s)->alloc = newlen;
            break;
        case SDS_TYPE_32:
            SDS_HDR(32,s)->alloc = newlen;
            break;
        case SDS_TYPE_64:
            SDS_HDR(64,s)->alloc = newlen;
            break;
    }
}

// Create a new sds string with specified content and length
static sds sdsnewlen(const void *init, size_t initlen) {
    void *sh;
    sds s;
    char type = sdsReqType(initlen);
    // Empty strings are usually created to append. Use type 8 since type 5
    // is not good at this.
    if (type == SDS_TYPE_5 && initlen == 0) type = SDS_TYPE_8;
    int hdrlen = sdsHdrSize(type);
    unsigned char *fp;

    sh = malloc(hdrlen+initlen+1);
    if (sh == NULL) return NULL;
    if (!init)
        memset(sh, 0, hdrlen+initlen+1);
    s = (char*)sh+hdrlen;
    fp = ((unsigned char*)s)-1;
    switch(type) {
        case SDS_TYPE_5: {
            *fp = type | (initlen << SDS_TYPE_BITS);
            break;
        }
        case SDS_TYPE_8: {
            SDS_HDR_VAR(8,s);
            sh->len = initlen;
            sh->alloc = initlen;
            *fp = type;
            break;
        }
        case SDS_TYPE_16: {
            SDS_HDR_VAR(16,s);
            sh->len = initlen;
            sh->alloc = initlen;
            *fp = type;
            break;
        }
        case SDS_TYPE_32: {
            SDS_HDR_VAR(32,s);
            sh->len = initlen;
            sh->alloc = initlen;
            *fp = type;
            break;
        }
        case SDS_TYPE_64: {
            SDS_HDR_VAR(64,s);
            sh->len = initlen;
            sh->alloc = initlen;
            *fp = type;
            break;
        }
    }
    if (initlen && init)
        memcpy(s, init, initlen);
    s[initlen] = '\0';
    return s;
}

// Create empty sds string
static sds sdsempty(void) {
    return sdsnewlen("", 0);
}

// Create sds from C string
static sds sdsnew(const char *init) {
    size_t initlen = (init == NULL) ? 0 : strlen(init);
    return sdsnewlen(init, initlen);
}

// Duplicate an sds string
static sds sdsdup(const sds s) {
    return sdsnewlen(s, sdslen(s));
}

// Free an sds string
static void sdsfree(sds s) {
    if (s == NULL) return;
    free((char*)s-sdsHdrSize(s[-1]));
}

// Make room for additional bytes
static sds sdsMakeRoomFor(sds s, size_t addlen) {
    void *sh, *newsh;
    size_t avail = sdsavail(s);
    size_t len, newlen, reqlen;
    char type, oldtype = s[-1] & SDS_TYPE_MASK;
    int hdrlen;

    if (avail >= addlen) return s;

    len = sdslen(s);
    sh = (char*)s-sdsHdrSize(oldtype);
    reqlen = newlen = (len+addlen);
    if (newlen < SDS_MAX_PREALLOC)
        newlen *= 2;
    else
        newlen += SDS_MAX_PREALLOC;

    type = sdsReqType(newlen);
    if (type == SDS_TYPE_5) type = SDS_TYPE_8;

    hdrlen = sdsHdrSize(type);
    if (oldtype==type) {
        newsh = realloc(sh, hdrlen+newlen+1);
        if (newsh == NULL) return NULL;
        s = (char*)newsh+hdrlen;
    } else {
        newsh = malloc(hdrlen+newlen+1);
        if (newsh == NULL) return NULL;
        memcpy((char*)newsh+hdrlen, s, len+1);
        free(sh);
        s = (char*)newsh+hdrlen;
        s[-1] = type;
        sdssetlen(s, len);
    }
    sdssetalloc(s, newlen);
    return s;
}

// Append binary-safe data to sds
static sds sdscatlen(sds s, const void *t, size_t len) {
    size_t curlen = sdslen(s);

    s = sdsMakeRoomFor(s, len);
    if (s == NULL) return NULL;
    memcpy(s+curlen, t, len);
    sdssetlen(s, curlen+len);
    s[curlen+len] = '\0';
    return s;
}

// Append C string to sds
static sds sdscat(sds s, const char *t) {
    return sdscatlen(s, t, strlen(t));
}

// Append sds to sds
static sds sdscatsds(sds s, const sds t) {
    return sdscatlen(s, t, sdslen(t));
}

// Copy C string to sds (destructive)
static sds sdscpylen(sds s, const char *t, size_t len) {
    if (sdsalloc(s) < len) {
        s = sdsMakeRoomFor(s, len-sdslen(s));
        if (s == NULL) return NULL;
    }
    memcpy(s, t, len);
    s[len] = '\0';
    sdssetlen(s, len);
    return s;
}

static sds sdscpy(sds s, const char *t) {
    return sdscpylen(s, t, strlen(t));
}

// Compare two sds strings
static int sdscmp(const sds s1, const sds s2) {
    size_t l1, l2, minlen;
    int cmp;

    l1 = sdslen(s1);
    l2 = sdslen(s2);
    minlen = (l1 < l2) ? l1 : l2;
    cmp = memcmp(s1, s2, minlen);
    if (cmp == 0) return l1>l2? 1: (l1<l2? -1: 0);
    return cmp;
}

// Grow the sds to specified length, zero-filling new bytes
static sds sdsgrowzero(sds s, size_t len) {
    size_t curlen = sdslen(s);

    if (len <= curlen) return s;
    s = sdsMakeRoomFor(s, len-curlen);
    if (s == NULL) return NULL;

    memset(s+curlen, 0, (len-curlen+1));
    sdssetlen(s, len);
    return s;
}

// Get substring (modifies in-place)
static void sdsrange(sds s, ssize_t start, ssize_t end) {
    size_t newlen, len = sdslen(s);

    if (len == 0) return;
    if (start < 0) {
        start = len+start;
        if (start < 0) start = 0;
    }
    if (end < 0) {
        end = len+end;
        if (end < 0) end = 0;
    }
    newlen = (start > end) ? 0 : (end-start)+1;
    if (newlen != 0) {
        if (start >= (ssize_t)len) {
            newlen = 0;
        } else if (end >= (ssize_t)len) {
            end = len-1;
            newlen = (end-start)+1;
        }
    }
    if (start && newlen) memmove(s, s+start, newlen);
    s[newlen] = 0;
    sdssetlen(s, newlen);
}

// Trim characters from both sides
static sds sdstrim(sds s, const char *cset) {
    char *end, *sp, *ep;
    size_t len;

    sp = s;
    ep = end = s+sdslen(s)-1;
    while(sp <= end && strchr(cset, *sp)) sp++;
    while(ep > sp && strchr(cset, *ep)) ep--;
    len = (ep-sp)+1;
    if (s != sp) memmove(s, sp, len);
    s[len] = '\0';
    sdssetlen(s, len);
    return s;
}

// Convert to lowercase
static void sdstolower(sds s) {
    size_t len = sdslen(s), j;
    for (j = 0; j < len; j++) s[j] = tolower(s[j]);
}

// Convert to uppercase
static void sdstoupper(sds s) {
    size_t len = sdslen(s), j;
    for (j = 0; j < len; j++) s[j] = toupper(s[j]);
}

// Clear the string (keep allocated memory)
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

// Strip whitespace from both ends (returns new bytes)
sds bytes_strip(const char* s) {
    sds result = sdsnew(s);
    if (result == NULL) return NULL;
    sdstrim(result, " \t\n\r\f\v");
    return result;
}

// Strip whitespace from left (returns new bytes)
sds bytes_lstrip(const char* s) {
    size_t len = strlen(s);
    const char *start = s;
    while (start < s + len && isspace(*start)) start++;
    return sdsnewlen(start, s + len - start);
}

// Strip whitespace from right (returns new bytes)
sds bytes_rstrip(const char* s) {
    size_t len = strlen(s);
    const char *end = s + len - 1;
    while (end >= s && isspace(*end)) end--;
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

// Center bytes in field of given width
sds bytes_center(const char* s, int64_t width) {
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

// Left-justify bytes in field of given width
sds bytes_ljust(const char* s, int64_t width) {
    size_t len = strlen(s);
    if ((size_t)width <= len) return sdsnew(s);

    sds result = sdsnewlen(NULL, width);
    if (result == NULL) return NULL;

    memcpy(result, s, len);
    memset(result + len, ' ', width - len);
    result[width] = '\0';

    return result;
}

// Right-justify bytes in field of given width
sds bytes_rjust(const char* s, int64_t width) {
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
