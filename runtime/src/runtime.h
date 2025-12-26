#ifndef RUNTIME_H
#define RUNTIME_H

// ============================================================================
// Python Compiler Runtime using musl libc
// Static linking - produces fully static executables
// Supports: x86_64, RISC-V 64-bit
// ============================================================================

#include "types.h"
#include "io.h"
#include "str.h"
#include "bytes.h"
#include "exception.h"

// ============================================================================
// List structure for list[int]
// ============================================================================

typedef struct {
    int64_t* data;
    int64_t len;
    int64_t cap;
} List;

List* LIST_METHOD(__init__)(void);
void LIST_METHOD(append)(List* list, int64_t value);
int64_t LIST_METHOD(__getitem__)(List* list, int64_t index);
void LIST_METHOD(__setitem__)(List* list, int64_t index, int64_t value);
int64_t LIST_METHOD(__len__)(List* list);
void LIST_METHOD(free)(List* list);
String* LIST_METHOD(__str__)(List* list);
String* LIST_METHOD(__repr__)(List* list);

// ============================================================================
// ListIterator structure
// ============================================================================

typedef struct {
    List* list;
    int64_t index;
} ListIterator;

ListIterator* LIST_METHOD(__iter__)(List* list);
ListIterator* LIST_ITERATOR_METHOD(__iter__)(ListIterator* iter);
int64_t LIST_ITERATOR_METHOD(__next__)(ListIterator* iter);
void LIST_ITERATOR_METHOD(__dealloc__)(ListIterator* iter);

// ============================================================================
// Range structure
// ============================================================================

typedef struct {
    int64_t start;
    int64_t stop;
    int64_t step;
    int64_t current;
} Range;

Range* __pyc___builtin___range_1(int64_t stop);
Range* __pyc___builtin___range_2(int64_t start, int64_t stop);
Range* __pyc___builtin___range_3(int64_t start, int64_t stop, int64_t step);

Range* RANGE_METHOD(__iter__)(Range* r);
int64_t RANGE_METHOD(__next__)(Range* r);
int64_t RANGE_METHOD(__len__)(Range* r);
void RANGE_METHOD(__dealloc__)(Range* r);
String* RANGE_METHOD(__str__)(Range* r);
String* RANGE_METHOD(__repr__)(Range* r);

// ============================================================================
// ByteArray structure
// ============================================================================

typedef struct {
    uint8_t* data;
    int64_t len;
    int64_t cap;
} ByteArray;

ByteArray* BYTEARRAY_METHOD(__init__)(void);
void BYTEARRAY_METHOD(append)(ByteArray* ba, int64_t value);
int64_t BYTEARRAY_METHOD(__getitem__)(ByteArray* ba, int64_t index);
void BYTEARRAY_METHOD(__setitem__)(ByteArray* ba, int64_t index, int64_t value);
int64_t BYTEARRAY_METHOD(__len__)(ByteArray* ba);
void BYTEARRAY_METHOD(free)(ByteArray* ba);
String* BYTEARRAY_METHOD(__str__)(ByteArray* ba);
String* BYTEARRAY_METHOD(__repr__)(ByteArray* ba);

// ============================================================================
// Class operations
// ============================================================================

void* class_new(int64_t size);

// ============================================================================
// I/O helpers for compiler
// ============================================================================

void write_str_impl(const char* str);
void write_string_impl(String* str);
void write_char_impl(char c);
void write_newline_impl(void);
void write_space_impl(void);
char* int64_to_str_impl(int64_t value, char* buffer);

void __pyc___builtin___int___print__(int64_t value);
void __pyc___builtin___bool___print__(int8_t value);

#endif // RUNTIME_H
