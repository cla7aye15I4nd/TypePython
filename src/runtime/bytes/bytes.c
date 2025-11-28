// TypePython Runtime Library - Bytes Module
// Builtin string/bytes operations

#include <stdint.h>
#include <string.h>
#include <stdlib.h>

// String concatenation - allocates new string
const char* strcat_bytes(const char* s1, const char* s2) {
    size_t len1 = strlen(s1);
    size_t len2 = strlen(s2);
    char* result = (char*)malloc(len1 + len2 + 1);
    strcpy(result, s1);
    strcat(result, s2);
    return result;
}

// String comparison - returns 1 if equal, 0 if not
int64_t strcmp_bytes(const char* s1, const char* s2) {
    return strcmp(s1, s2) == 0 ? 1 : 0;
}
