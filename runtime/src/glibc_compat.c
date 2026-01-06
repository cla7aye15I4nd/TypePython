// Compatibility layer for glibc fortified functions
// These are stubs that call the regular versions
// Required because system ICU was compiled with glibc

#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <unistd.h>

// Memory functions
void* __memcpy_chk(void* dest, const void* src, size_t n, size_t destlen) {
    (void)destlen;  // Ignore buffer size check
    return memcpy(dest, src, n);
}

void* __memmove_chk(void* dest, const void* src, size_t n, size_t destlen) {
    (void)destlen;
    return memmove(dest, src, n);
}

void* __memset_chk(void* s, int c, size_t n, size_t slen) {
    (void)slen;
    return memset(s, c, n);
}

// String functions
char* __strcpy_chk(char* dest, const char* src, size_t destlen) {
    (void)destlen;
    return strcpy(dest, src);
}

char* __strncpy_chk(char* dest, const char* src, size_t n, size_t destlen) {
    (void)destlen;
    return strncpy(dest, src, n);
}

char* __strcat_chk(char* dest, const char* src, size_t destlen) {
    (void)destlen;
    return strcat(dest, src);
}

char* __strncat_chk(char* dest, const char* src, size_t n, size_t destlen) {
    (void)destlen;
    return strncat(dest, src, n);
}

// File I/O functions
size_t __fread_chk(void* ptr, size_t size, size_t nmemb, FILE* stream, size_t ptrlen) {
    (void)ptrlen;
    return fread(ptr, size, nmemb, stream);
}

int __fprintf_chk(FILE* stream, int flag, const char* format, ...) {
    (void)flag;
    va_list args;
    va_start(args, format);
    int ret = vfprintf(stream, format, args);
    va_end(args);
    return ret;
}

int __vfprintf_chk(FILE* stream, int flag, const char* format, va_list args) {
    (void)flag;
    return vfprintf(stream, format, args);
}

int __sprintf_chk(char* str, int flag, size_t strlen, const char* format, ...) {
    (void)flag;
    (void)strlen;
    va_list args;
    va_start(args, format);
    int ret = vsprintf(str, format, args);
    va_end(args);
    return ret;
}

int __snprintf_chk(char* str, size_t maxlen, int flag, size_t strlen, const char* format, ...) {
    (void)flag;
    (void)strlen;
    va_list args;
    va_start(args, format);
    int ret = vsnprintf(str, maxlen, format, args);
    va_end(args);
    return ret;
}

// Path functions
char* __realpath_chk(const char* path, char* resolved_path, size_t resolved_len) {
    (void)resolved_len;
    return realpath(path, resolved_path);
}

// strtol functions (for glibc 2.38+)
long int __isoc23_strtol(const char* nptr, char** endptr, int base) {
    return strtol(nptr, endptr, base);
}

unsigned long int __isoc23_strtoul(const char* nptr, char** endptr, int base) {
    return strtoul(nptr, endptr, base);
}

// ============================================================================
// libstdc++ compatibility symbols (needed for static linking with musl)
// ============================================================================

#include <wchar.h>
#include <fcntl.h>
#include <sys/random.h>

// DSO handle (required for static linking)
void* __dso_handle = 0;

// Thread safety indicator (we're single-threaded in our use case)
char __libc_single_threaded = 1;

// Wide character functions
wchar_t* __wmemcpy_chk(wchar_t* dest, const wchar_t* src, size_t n, size_t destlen) {
    (void)destlen;
    return wmemcpy(dest, src, n);
}

wchar_t* __wmemset_chk(wchar_t* dest, wchar_t c, size_t n, size_t destlen) {
    (void)destlen;
    return wmemset(dest, c, n);
}

size_t __mbsrtowcs_chk(wchar_t* dest, const char** src, size_t len, mbstate_t* ps, size_t destlen) {
    (void)destlen;
    return mbsrtowcs(dest, src, len, ps);
}

size_t __mbsnrtowcs_chk(wchar_t* dest, const char** src, size_t nmc, size_t len, mbstate_t* ps, size_t destlen) {
    (void)destlen;
    return mbsnrtowcs(dest, src, nmc, len, ps);
}

// Read function
ssize_t __read_chk(int fd, void* buf, size_t nbytes, size_t buflen) {
    (void)buflen;
    return read(fd, buf, nbytes);
}

// arc4random (use getrandom as fallback)
unsigned int arc4random(void) {
    unsigned int val;
    if (getrandom(&val, sizeof(val), 0) < 0) {
        // Fallback - not cryptographically secure but prevents crash
        static unsigned int seed = 12345;
        seed = seed * 1103515245 + 12345;
        return seed;
    }
    return val;
}

// Dynamic linker find_object (stub - not needed for static linking)
struct dl_find_object {
    unsigned long long dlfo_flags;
    void *dlfo_map_start;
    void *dlfo_map_end;
    struct link_map *dlfo_link_map;
    void *dlfo_eh_frame;
};

int _dl_find_object(void *address, struct dl_find_object *result) {
    (void)address;
    (void)result;
    return -1;  // Not found (signal that we don't have this info)
}
