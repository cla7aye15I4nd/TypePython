// TypePython Runtime Library - Tuple Module
// Immutable fixed-size array implementation for tuple type
//
// Memory layout: PyTuple struct with length and data array
// Tuples are immutable - no modification after creation

#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

// ============================================================================
// Tuple Data Structure
// ============================================================================

typedef struct {
    int64_t len;       // Number of elements
    int64_t* data;     // Pointer to element array
} PyTuple;

// ============================================================================
// Tuple Core Functions
// ============================================================================

// Create a new tuple with specified length
PyTuple* tuple_new(int64_t len) {
    PyTuple* tuple = (PyTuple*)malloc(sizeof(PyTuple));
    if (tuple == NULL) return NULL;
    tuple->data = (int64_t*)malloc(len * sizeof(int64_t));
    if (tuple->data == NULL && len > 0) {
        free(tuple);
        return NULL;
    }
    tuple->len = len;
    return tuple;
}

// Set item at index (used during construction only)
void tuple_setitem(PyTuple* tuple, int64_t index, int64_t value) {
    if (tuple == NULL || index < 0 || index >= tuple->len) return;
    tuple->data[index] = value;
}

// Get the length of the tuple
int64_t tuple_len(PyTuple* tuple) {
    if (tuple == NULL) return 0;
    return tuple->len;
}

// Normalize negative index to positive
static int64_t normalize_index(int64_t index, int64_t len) {
    if (index < 0) {
        index = len + index;
    }
    return index;
}

// Get item at index (returns 0 if out of bounds)
int64_t tuple_getitem(PyTuple* tuple, int64_t index) {
    if (tuple == NULL) return 0;
    index = normalize_index(index, tuple->len);
    if (index < 0 || index >= tuple->len) {
        return 0;
    }
    return tuple->data[index];
}

// ============================================================================
// Tuple Comparison Functions
// ============================================================================

// Check if two tuples are equal
int64_t tuple_eq(PyTuple* a, PyTuple* b) {
    if (a == NULL || b == NULL) return a == b ? 1 : 0;
    if (a->len != b->len) return 0;
    for (int64_t i = 0; i < a->len; i++) {
        if (a->data[i] != b->data[i]) return 0;
    }
    return 1;
}

// Check if two tuples are not equal
int64_t tuple_ne(PyTuple* a, PyTuple* b) {
    return !tuple_eq(a, b);
}

// ============================================================================
// Tuple Print Functions
// ============================================================================

// Print PyTuple of ints (note: different from print_tuple_int in list.c which prints PyList-based tuples)
void print_pytuple_int(PyTuple* tuple) {
    printf("(");
    if (tuple != NULL) {
        for (int64_t i = 0; i < tuple->len; i++) {
            if (i > 0) printf(", ");
            printf("%ld", tuple->data[i]);
        }
        // Single element tuple needs trailing comma
        if (tuple->len == 1) printf(",");
    }
    printf(")");
}

// Print PyTuple of floats (note: different from print_tuple_float in list.c which prints PyList-based tuples)
void print_pytuple_float(PyTuple* tuple) {
    printf("(");
    if (tuple != NULL) {
        double* data = (double*)tuple->data;
        for (int64_t i = 0; i < tuple->len; i++) {
            if (i > 0) printf(", ");
            double val = data[i];
            // Match Python's float printing behavior
            if (val == (int64_t)val && val < 1e16 && val > -1e16) {
                printf("%.1f", val);
            } else {
                printf("%g", val);
            }
        }
        // Single element tuple needs trailing comma
        if (tuple->len == 1) printf(",");
    }
    printf(")");
}

// ============================================================================
// Tuple Iteration Support
// ============================================================================

// Iterator structure for tuples
typedef struct {
    PyTuple* tuple;
    int64_t current;
} PyTupleIter;

// Create an iterator for the tuple
PyTupleIter* tuple_iter(PyTuple* tuple) {
    PyTupleIter* iter = (PyTupleIter*)malloc(sizeof(PyTupleIter));
    if (iter == NULL) return NULL;
    iter->tuple = tuple;
    iter->current = 0;
    return iter;
}

// Get next value from iterator
// Returns 1 if value is valid, 0 if iteration is complete
int64_t tuple_iter_next(PyTupleIter* iter, int64_t* out_value) {
    if (iter == NULL || iter->tuple == NULL) return 0;
    if (iter->current >= iter->tuple->len) return 0;
    *out_value = iter->tuple->data[iter->current];
    iter->current++;
    return 1;
}

// Free the iterator (not the tuple itself)
void tuple_iter_free(PyTupleIter* iter) {
    if (iter != NULL) {
        free(iter);
    }
}

// ============================================================================
// Tuple Utility Functions
// ============================================================================

// Create a copy of the tuple
PyTuple* tuple_copy(PyTuple* tuple) {
    if (tuple == NULL) return NULL;
    PyTuple* copy = tuple_new(tuple->len);
    if (copy == NULL) return NULL;
    memcpy(copy->data, tuple->data, tuple->len * sizeof(int64_t));
    return copy;
}

// Check if value is in tuple
int64_t tuple_contains(PyTuple* tuple, int64_t value) {
    if (tuple == NULL) return 0;
    for (int64_t i = 0; i < tuple->len; i++) {
        if (tuple->data[i] == value) return 1;
    }
    return 0;
}

// Count occurrences of value in tuple
int64_t tuple_count(PyTuple* tuple, int64_t value) {
    if (tuple == NULL) return 0;
    int64_t count = 0;
    for (int64_t i = 0; i < tuple->len; i++) {
        if (tuple->data[i] == value) count++;
    }
    return count;
}

// Find index of first occurrence of value (-1 if not found)
int64_t tuple_index(PyTuple* tuple, int64_t value) {
    if (tuple == NULL) return -1;
    for (int64_t i = 0; i < tuple->len; i++) {
        if (tuple->data[i] == value) return i;
    }
    return -1;
}

// Tuple concatenation
PyTuple* tuple_concat(PyTuple* a, PyTuple* b) {
    if (a == NULL && b == NULL) return tuple_new(0);
    if (a == NULL) return tuple_copy(b);
    if (b == NULL) return tuple_copy(a);

    int64_t new_len = a->len + b->len;
    PyTuple* result = tuple_new(new_len);
    if (result == NULL) return NULL;

    memcpy(result->data, a->data, a->len * sizeof(int64_t));
    memcpy(result->data + a->len, b->data, b->len * sizeof(int64_t));
    return result;
}

// Tuple repetition (tuple * n)
PyTuple* tuple_repeat(PyTuple* tuple, int64_t n) {
    if (tuple == NULL || n <= 0) return tuple_new(0);

    int64_t new_len = tuple->len * n;
    PyTuple* result = tuple_new(new_len);
    if (result == NULL) return NULL;

    for (int64_t i = 0; i < n; i++) {
        memcpy(result->data + i * tuple->len, tuple->data, tuple->len * sizeof(int64_t));
    }
    return result;
}

// ============================================================================
// divmod - returns a PyTuple with (quotient, remainder)
// ============================================================================

#include <math.h>

PyTuple* divmod_int(int64_t a, int64_t b) {
    PyTuple* result = tuple_new(2);
    if (result == NULL) return NULL;

    // Python-style floor division and modulo
    int64_t q = a / b;
    int64_t r = a % b;

    // Adjust for Python semantics (floor division)
    if ((r != 0) && ((a < 0) != (b < 0))) {
        q -= 1;
        r += b;
    }

    result->data[0] = q;
    result->data[1] = r;

    return result;
}

// divmod for floats - returns a PyTuple with (quotient, remainder) stored as doubles
PyTuple* divmod_float(double a, double b) {
    PyTuple* result = tuple_new(2);
    if (result == NULL) return NULL;

    // Python-style floor division and modulo for floats
    double q = floor(a / b);
    double r = a - q * b;

    // Store doubles in the int64_t array (type punning)
    double* data = (double*)result->data;
    data[0] = q;
    data[1] = r;

    return result;
}

// ============================================================================
// tuple() constructor from iterables
// ============================================================================

// Forward declarations for list
typedef struct {
    int64_t len;
    int64_t capacity;
    int64_t* data;
} PyListTuple;

// Forward declarations for string list
typedef struct {
    int64_t len;
    int64_t capacity;
    char** data;
} PyStrListTuple;

// Forward declaration for set
#define SET_OCCUPIED_TUPLE 1
typedef struct {
    int64_t key;
    uint8_t state;
} SetEntryTuple;

typedef struct {
    int64_t len;
    int64_t capacity;
    SetEntryTuple* entries;
} PySetTuple;

// Forward declaration for dict
#define DICT_OCCUPIED_TUPLE 1
typedef struct {
    int64_t key;
    int64_t value;
    uint8_t state;
} DictEntryTuple;

typedef struct {
    int64_t len;
    int64_t capacity;
    DictEntryTuple* entries;
} PyDictTuple;

// Create tuple from string (character ordinals)
PyTuple* tuple_from_str(const char* s) {
    if (s == NULL) return tuple_new(0);
    size_t len = strlen(s);
    PyTuple* result = tuple_new(len);
    if (result == NULL) return NULL;
    for (size_t i = 0; i < len; i++) {
        result->data[i] = (int64_t)(unsigned char)s[i];
    }
    return result;
}

// Create tuple from bytes (byte values)
PyTuple* tuple_from_bytes(const char* s) {
    if (s == NULL) return tuple_new(0);
    size_t len = strlen(s);
    PyTuple* result = tuple_new(len);
    if (result == NULL) return NULL;
    for (size_t i = 0; i < len; i++) {
        result->data[i] = (int64_t)(unsigned char)s[i];
    }
    return result;
}

// Create tuple from list of ints
PyTuple* tuple_from_list(PyListTuple* list) {
    if (list == NULL) return tuple_new(0);
    PyTuple* result = tuple_new(list->len);
    if (result == NULL) return NULL;
    memcpy(result->data, list->data, list->len * sizeof(int64_t));
    return result;
}

// Create tuple from set of ints
PyTuple* tuple_from_set(PySetTuple* set) {
    if (set == NULL || set->len == 0) return tuple_new(0);
    PyTuple* result = tuple_new(set->len);
    if (result == NULL) return NULL;
    int64_t j = 0;
    for (int64_t i = 0; i < set->capacity && j < set->len; i++) {
        if (set->entries[i].state == SET_OCCUPIED_TUPLE) {
            result->data[j++] = set->entries[i].key;
        }
    }
    return result;
}

// Create tuple from dict keys (int keys)
PyTuple* tuple_from_dict(PyDictTuple* dict) {
    if (dict == NULL || dict->len == 0) return tuple_new(0);
    PyTuple* result = tuple_new(dict->len);
    if (result == NULL) return NULL;
    int64_t j = 0;
    for (int64_t i = 0; i < dict->capacity && j < dict->len; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED_TUPLE) {
            result->data[j++] = dict->entries[i].key;
        }
    }
    return result;
}

// ============================================================================
// Tuple reversed - create a new reversed tuple
// ============================================================================

// Create a reversed copy of the tuple
PyTuple* tuple_reversed(PyTuple* tuple) {
    if (tuple == NULL) return tuple_new(0);
    PyTuple* result = tuple_new(tuple->len);
    if (result == NULL) return NULL;
    for (int64_t i = 0; i < tuple->len; i++) {
        result->data[i] = tuple->data[tuple->len - 1 - i];
    }
    return result;
}
