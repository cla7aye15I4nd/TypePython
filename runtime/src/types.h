#ifndef TYPES_H
#define TYPES_H

// ============================================================================
// Standard type definitions via musl libc
// ============================================================================

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

// ============================================================================
// Method naming macros
// ============================================================================

// Generic macro for builtin method names
#define BUILTIN_METHOD(type, name) __pyc___builtin___##type##_##name

// Type-specific method macros
#define LIST_METHOD(name)          BUILTIN_METHOD(list, name)
#define LIST_ITERATOR_METHOD(name) BUILTIN_METHOD(list_iterator, name)
#define BYTEARRAY_METHOD(name)     BUILTIN_METHOD(bytearray, name)
#define BYTES_METHOD(name)         BUILTIN_METHOD(bytes, name)
#define STR_METHOD(name)           BUILTIN_METHOD(str, name)
#define RANGE_METHOD(name)         BUILTIN_METHOD(range, name)
#define EXCEPTION_METHOD(name)     BUILTIN_METHOD(Exception, name)
#define STOPITERATION_METHOD(name) BUILTIN_METHOD(StopIteration, name)

#endif // TYPES_H
