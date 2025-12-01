// TypePython Runtime Library - List Module
// Dynamic array implementation for list type
//
// Memory layout: PyList struct with flexible array member
// Assignment creates a copy (not alias) for safety with realloc

#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <math.h>
#include <ctype.h>

#include "../common.h"

// ============================================================================
// List Data Structure
// ============================================================================

typedef struct {
    int64_t len;       // Current number of elements
    int64_t capacity;  // Allocated capacity
    int64_t* data;     // Pointer to element array (separate allocation)
} PyList;

// String list - stores char* pointers instead of int64_t
typedef struct {
    int64_t len;       // Current number of elements
    int64_t capacity;  // Allocated capacity
    char** data;       // Pointer to string pointer array
} PyStrList;

#define INITIAL_CAPACITY 8

// ============================================================================
// Internal Helper Functions
// ============================================================================

// Create a single-character string using malloc
static char* local_str_char_at(const char* s, int64_t index) {
    if (!s) return NULL;
    size_t len = strlen(s);
    if (index < 0) index = len + index;
    if (index < 0 || (size_t)index >= len) return NULL;
    char* result = (char*)malloc(2);
    if (!result) return NULL;
    result[0] = s[index];
    result[1] = '\0';
    return result;
}

static PyList* list_alloc(int64_t capacity) {
    if (capacity < INITIAL_CAPACITY) capacity = INITIAL_CAPACITY;
    PyList* list = (PyList*)malloc(sizeof(PyList));
    if (!list) return NULL;
    list->data = (int64_t*)malloc(capacity * sizeof(int64_t));
    if (!list->data) { free(list); return NULL; }
    list->len = 0;
    list->capacity = capacity;
    return list;
}

static void list_grow(PyList* list, int64_t min_capacity) {
    int64_t new_capacity = list->capacity * 2;
    if (new_capacity < min_capacity) new_capacity = min_capacity;
    int64_t* new_data = (int64_t*)realloc(list->data, new_capacity * sizeof(int64_t));
    if (!new_data) return;
    list->data = new_data;
    list->capacity = new_capacity;
}

// ============================================================================
// List Core Functions
// ============================================================================

// Create a new empty list
PyList* list_new(void) {
    return list_alloc(INITIAL_CAPACITY);
}

// Create a list with pre-allocated capacity
PyList* list_with_capacity(int64_t capacity) {
    return list_alloc(capacity);
}

// Get the length of the list
int64_t list_len(PyList* list) {
    if (list == NULL) return 0;
    return list->len;
}

// Get item at index (returns 0 if out of bounds - caller should check)
int64_t list_getitem(PyList* list, int64_t index) {
    if (list == NULL) return 0;
    index = tpy_normalize_index(index, list->len);
    if (index < 0 || index >= list->len) {
        // TODO: Error handling - for now return 0
        return 0;
    }
    return list->data[index];
}

// Set item at index
void list_setitem(PyList* list, int64_t index, int64_t value) {
    if (list == NULL) return;
    index = tpy_normalize_index(index, list->len);
    if (index < 0 || index >= list->len) {
        // TODO: Error handling - for now silently fail
        return;
    }
    list->data[index] = value;
}

// Append an item to the list (mutates in place, returns void like Python)
void list_append(PyList* list, int64_t value) {
    if (list == NULL) return;
    if (list->len >= list->capacity) {
        list_grow(list, list->len + 1);
    }
    list->data[list->len++] = value;
}

// Delete item at specific index (mutates in place, returns void)
// This is used by the del statement: del list[index]
void list_delitem(PyList* list, int64_t index) {
    if (list == NULL || list->len == 0) {
        return;
    }

    index = tpy_normalize_index(index, list->len);
    if (index < 0 || index >= list->len) {
        return;
    }

    // Shift elements left to fill the gap
    for (int64_t i = index; i < list->len - 1; i++) {
        list->data[i] = list->data[i + 1];
    }
    list->len--;
}

// Pop item at specific index (mutates in place, returns the value)
// If index is -1, pops from the end (default behavior)
int64_t list_pop(PyList* list, int64_t index) {
    if (list == NULL || list->len == 0) {
        // TODO: Error handling
        return 0;
    }

    // Default: pop from end
    if (index == -1) {
        return list->data[--list->len];
    }

    index = tpy_normalize_index(index, list->len);
    if (index < 0 || index >= list->len) {
        // TODO: Error handling for out of bounds
        return 0;
    }

    int64_t value = list->data[index];
    // Shift elements left to fill the gap
    for (int64_t i = index; i < list->len - 1; i++) {
        list->data[i] = list->data[i + 1];
    }
    list->len--;
    return value;
}

// Insert an item at index (mutates in place, returns void like Python)
void list_insert(PyList* list, int64_t index, int64_t value) {
    if (list == NULL) return;

    index = tpy_normalize_index(index, list->len);
    if (index < 0) index = 0;
    if (index > list->len) index = list->len;

    if (list->len >= list->capacity) {
        list_grow(list, list->len + 1);
    }

    // Shift elements to the right
    memmove(&list->data[index + 1], &list->data[index],
            (list->len - index) * sizeof(int64_t));
    list->data[index] = value;
    list->len++;
}

// Remove the first occurrence of value
void list_remove(PyList* list, int64_t value) {
    if (list == NULL) return;
    for (int64_t i = 0; i < list->len; i++) {
        if (list->data[i] == value) {
            // Shift elements to the left
            memmove(&list->data[i], &list->data[i + 1],
                    (list->len - i - 1) * sizeof(int64_t));
            list->len--;
            return;
        }
    }
}

// Clear all items from the list
void list_clear(PyList* list) {
    if (list == NULL) return;
    list->len = 0;
}

// Create a shallow copy of the list
PyList* list_copy(PyList* list) {
    if (list == NULL) return list_new();
    PyList* copy = list_alloc(list->capacity);
    if (copy == NULL) return NULL;
    copy->len = list->len;
    memcpy(copy->data, list->data, list->len * sizeof(int64_t));
    return copy;
}

// Extend list with another list (mutates in place, returns void like Python)
void list_extend(PyList* list, PyList* other) {
    if (list == NULL) return;
    if (other == NULL || other->len == 0) return;

    int64_t new_len = list->len + other->len;
    if (new_len > list->capacity) {
        list_grow(list, new_len);
    }

    memcpy(&list->data[list->len], other->data, other->len * sizeof(int64_t));
    list->len = new_len;
}

// Reverse the list in place
void list_reverse(PyList* list) {
    if (list == NULL || list->len <= 1) return;
    int64_t i = 0;
    int64_t j = list->len - 1;
    while (i < j) {
        int64_t temp = list->data[i];
        list->data[i] = list->data[j];
        list->data[j] = temp;
        i++;
        j--;
    }
}

// Find the index of value (returns -1 if not found)
int64_t list_index(PyList* list, int64_t value) {
    if (list == NULL) return -1;
    for (int64_t i = 0; i < list->len; i++) {
        if (list->data[i] == value) {
            return i;
        }
    }
    return -1;
}

// Count occurrences of value
int64_t list_count(PyList* list, int64_t value) {
    if (list == NULL) return 0;
    int64_t count = 0;
    for (int64_t i = 0; i < list->len; i++) {
        if (list->data[i] == value) {
            count++;
        }
    }
    return count;
}

// Check if list contains value
int64_t list_contains(PyList* list, int64_t value) {
    return list_index(list, value) >= 0 ? 1 : 0;
}

// Check if list contains float value (compares float to int values)
// Python: 1.0 in [1, 2, 3] == True because 1.0 == 1
int64_t list_contains_float(PyList* list, double value) {
    if (list == NULL) return 0;
    // List contains int64_t values, compare float to int
    for (int64_t i = 0; i < list->len; i++) {
        if ((double)list->data[i] == value) return 1;
    }
    return 0;
}

// ============================================================================
// List Slicing
// ============================================================================

// Slice list[start:end]
PyList* list_slice(PyList* list, int64_t start, int64_t end) {
    if (list == NULL) return list_new();

    // Handle negative indices
    if (start < 0) start = list->len + start;
    if (end < 0) end = list->len + end;

    // Clamp to bounds
    if (start < 0) start = 0;
    if (end < 0) end = 0;
    if (start > list->len) start = list->len;
    if (end > list->len) end = list->len;

    // Handle empty slice
    if (start >= end) return list_new();

    int64_t slice_len = end - start;
    PyList* result = list_alloc(slice_len);
    if (result == NULL) return NULL;

    memcpy(result->data, &list->data[start], slice_len * sizeof(int64_t));
    result->len = slice_len;
    return result;
}

// Slice with step: list[start:end:step]
PyList* list_slice_step(PyList* list, int64_t start, int64_t end, int64_t step) {
    if (list == NULL || step == 0) return list_new();

    int64_t len = list->len;

    // Handle default values (INT64_MAX used as sentinel)
    if (start == INT64_MAX) {
        start = (step > 0) ? 0 : len - 1;
    }
    if (end == INT64_MAX) {
        end = (step > 0) ? len : -len - 1;
    }

    if (step > 0) {
        // Forward slice
        if (start < 0) start = len + start;
        if (end < 0) end = len + end;
        if (start < 0) start = 0;
        if (end < 0) end = 0;
        if (start > len) start = len;
        if (end > len) end = len;
        if (start >= end) return list_new();

        // Calculate result length
        int64_t result_len = (end - start + step - 1) / step;
        PyList* result = list_alloc(result_len);
        if (result == NULL) return NULL;

        int64_t j = 0;
        for (int64_t i = start; i < end && j < result_len; i += step) {
            result->data[j++] = list->data[i];
        }
        result->len = j;
        return result;
    } else {
        // Negative step (reverse slice)
        if (start < 0) start = len + start;
        if (end < 0) end = len + end;

        if (start < 0) return list_new();
        if (start >= len) start = len - 1;
        if (end < -1) end = -1;
        if (end >= len) end = len - 1;

        if (start <= end) return list_new();

        // Calculate result length
        int64_t result_len = (start - end + (-step) - 1) / (-step);
        PyList* result = list_alloc(result_len);
        if (result == NULL) return NULL;

        int64_t j = 0;
        for (int64_t i = start; i > end && j < result_len; i += step) {
            result->data[j++] = list->data[i];
        }
        result->len = j;
        return result;
    }
}

// ============================================================================
// List Operations
// ============================================================================

// Concatenate two lists: list1 + list2
PyList* list_concat(PyList* list1, PyList* list2) {
    int64_t len1 = list1 ? list1->len : 0;
    int64_t len2 = list2 ? list2->len : 0;

    PyList* result = list_alloc(len1 + len2);
    if (result == NULL) return NULL;

    if (len1 > 0) {
        memcpy(result->data, list1->data, len1 * sizeof(int64_t));
    }
    if (len2 > 0) {
        memcpy(&result->data[len1], list2->data, len2 * sizeof(int64_t));
    }
    result->len = len1 + len2;
    return result;
}

// Repeat list n times: list * n
PyList* list_repeat(PyList* list, int64_t n) {
    if (list == NULL || n <= 0) return list_new();

    int64_t new_len = list->len * n;
    PyList* result = list_alloc(new_len);
    if (result == NULL) return NULL;

    for (int64_t i = 0; i < n; i++) {
        memcpy(&result->data[i * list->len], list->data, list->len * sizeof(int64_t));
    }
    result->len = new_len;
    return result;
}

// Compare two lists lexicographically
// Returns: -1 if list1 < list2, 0 if equal, 1 if list1 > list2
int64_t list_cmp(PyList* list1, PyList* list2) {
    int64_t len1 = list1 ? list1->len : 0;
    int64_t len2 = list2 ? list2->len : 0;
    int64_t min_len = len1 < len2 ? len1 : len2;

    // Compare element by element
    for (int64_t i = 0; i < min_len; i++) {
        if (list1->data[i] < list2->data[i]) return -1;
        if (list1->data[i] > list2->data[i]) return 1;
    }

    // All compared elements are equal, compare by length
    if (len1 < len2) return -1;
    if (len1 > len2) return 1;
    return 0;
}

// Check equality of two lists
int64_t list_eq(PyList* list1, PyList* list2) {
    int64_t len1 = list1 ? list1->len : 0;
    int64_t len2 = list2 ? list2->len : 0;

    if (len1 != len2) return 0;
    if (len1 == 0) return 1;

    return memcmp(list1->data, list2->data, len1 * sizeof(int64_t)) == 0 ? 1 : 0;
}

// Comparison function for qsort
static int cmp_int64(const void* a, const void* b) {
    int64_t va = *(const int64_t*)a;
    int64_t vb = *(const int64_t*)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

// Sort the list in place (ascending order)
void list_sort(PyList* list) {
    if (list == NULL || list->len <= 1) return;
    qsort(list->data, list->len, sizeof(int64_t), cmp_int64);
}

// Find the maximum element in the list
int64_t list_max(PyList* list) {
    if (list == NULL || list->len == 0) return 0;
    int64_t max_val = list->data[0];
    for (int64_t i = 1; i < list->len; i++) {
        if (list->data[i] > max_val) {
            max_val = list->data[i];
        }
    }
    return max_val;
}

// Find the minimum element in the list
int64_t list_min(PyList* list) {
    if (list == NULL || list->len == 0) return 0;
    int64_t min_val = list->data[0];
    for (int64_t i = 1; i < list->len; i++) {
        if (list->data[i] < min_val) {
            min_val = list->data[i];
        }
    }
    return min_val;
}

// ============================================================================
// Print Support
// ============================================================================

void print_list(PyList* list) {
    printf("[");
    if (list != NULL) {
        for (int64_t i = 0; i < list->len; i++) {
            if (i > 0) printf(", ");
            printf("%ld", list->data[i]);
        }
    }
    printf("]");
}

// Print list of floats
void print_list_float(PyList* list) {
    printf("[");
    if (list != NULL) {
        double* data = (double*)list->data;
        for (int64_t i = 0; i < list->len; i++) {
            if (i > 0) printf(", ");
            // Print with .0 suffix for whole numbers like Python
            double val = data[i];
            if (val == (int64_t)val && val >= -9007199254740992.0 && val <= 9007199254740992.0) {
                printf("%.1f", val);
            } else {
                printf("%g", val);
            }
        }
    }
    printf("]");
}

// Print list of lists (nested list)
void print_list_list(PyList* list) {
    printf("[");
    if (list != NULL) {
        for (int64_t i = 0; i < list->len; i++) {
            if (i > 0) printf(", ");
            // Each element is a pointer to another list, stored as int64
            PyList* inner = (PyList*)(uintptr_t)list->data[i];
            print_list(inner);  // Recursive call to print inner list
        }
    }
    printf("]");
}

// ============================================================================
// String List (PyStrList) Functions
// ============================================================================

// Allocate a new string list
static PyStrList* str_list_alloc(int64_t capacity) {
    if (capacity < INITIAL_CAPACITY) {
        capacity = INITIAL_CAPACITY;
    }
    PyStrList* list = (PyStrList*)malloc(sizeof(PyStrList));
    if (list == NULL) return NULL;
    list->data = (char**)malloc(capacity * sizeof(char*));
    if (list->data == NULL) {
        free(list);
        return NULL;
    }
    list->len = 0;
    list->capacity = capacity;
    return list;
}

// Create a new empty string list
PyStrList* str_list_new(void) {
    return str_list_alloc(INITIAL_CAPACITY);
}

// Create string list from a string (each char becomes a single-char string)
PyStrList* str_list_from_str(const char* s) {
    if (s == NULL) return str_list_new();

    size_t len = strlen(s);
    PyStrList* result = str_list_alloc((int64_t)len);
    if (result == NULL) return NULL;

    for (size_t i = 0; i < len; i++) {
        // Create a single-character string for each char
        char* single = (char*)malloc(2);
        if (single == NULL) {
            // Clean up on failure
            for (size_t j = 0; j < i; j++) {
                free(result->data[j]);
            }
            free(result->data);
            free(result);
            return NULL;
        }
        single[0] = s[i];
        single[1] = '\0';
        result->data[i] = single;
    }
    result->len = (int64_t)len;

    return result;
}

// Get length of string list
int64_t str_list_len(PyStrList* list) {
    if (list == NULL) return 0;
    return list->len;
}

// Print string list: ['h', 'e', 'l', 'l', 'o']
void print_str_list(PyStrList* list) {
    printf("[");
    if (list != NULL) {
        for (int64_t i = 0; i < list->len; i++) {
            if (i > 0) printf(", ");
            printf("'%s'", list->data[i] ? list->data[i] : "");
        }
    }
    printf("]");
}

// Sort string list (alphabetically)
PyStrList* str_list_sorted(PyStrList* list) {
    if (list == NULL) return str_list_new();

    PyStrList* result = str_list_alloc(list->len);
    if (result == NULL) return NULL;

    // Copy string pointers (shallow copy for now - they're single chars)
    for (int64_t i = 0; i < list->len; i++) {
        size_t slen = strlen(list->data[i]);
        result->data[i] = (char*)malloc(slen + 1);
        if (result->data[i] == NULL) {
            for (int64_t j = 0; j < i; j++) free(result->data[j]);
            free(result->data);
            free(result);
            return NULL;
        }
        strcpy(result->data[i], list->data[i]);
    }
    result->len = list->len;

    // Simple bubble sort for strings
    for (int64_t i = 0; i < result->len - 1; i++) {
        for (int64_t j = 0; j < result->len - i - 1; j++) {
            if (strcmp(result->data[j], result->data[j+1]) > 0) {
                char* tmp = result->data[j];
                result->data[j] = result->data[j+1];
                result->data[j+1] = tmp;
            }
        }
    }

    return result;
}

// Sort string list in place (alphabetically)
void str_list_sort(PyStrList* list) {
    if (list == NULL || list->len <= 1) return;

    // Simple bubble sort for strings
    for (int64_t i = 0; i < list->len - 1; i++) {
        for (int64_t j = 0; j < list->len - i - 1; j++) {
            if (strcmp(list->data[j], list->data[j+1]) > 0) {
                char* tmp = list->data[j];
                list->data[j] = list->data[j+1];
                list->data[j+1] = tmp;
            }
        }
    }
}

// ============================================================================
// Builtin Functions: sum, sorted, reversed
// ============================================================================

// Sum all elements in the list (for int lists)
int64_t list_sum(PyList* list, int64_t start) {
    int64_t sum = start;
    if (list != NULL) {
        for (int64_t i = 0; i < list->len; i++) {
            sum += list->data[i];
        }
    }
    return sum;
}

// Helper for qsort
static int compare_int64(const void* a, const void* b) {
    int64_t va = *(const int64_t*)a;
    int64_t vb = *(const int64_t*)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

// Return a new sorted list (ascending order)
PyList* list_sorted(PyList* list) {
    if (list == NULL) return list_new();

    PyList* result = list_with_capacity(list->len);
    if (result == NULL) return NULL;

    // Copy elements
    for (int64_t i = 0; i < list->len; i++) {
        result->data[i] = list->data[i];
    }
    result->len = list->len;

    // Sort
    qsort(result->data, result->len, sizeof(int64_t), compare_int64);

    return result;
}

// Return a new reversed list
PyList* list_reversed(PyList* list) {
    if (list == NULL) return list_new();

    PyList* result = list_with_capacity(list->len);
    if (result == NULL) return NULL;

    for (int64_t i = 0; i < list->len; i++) {
        result->data[i] = list->data[list->len - 1 - i];
    }
    result->len = list->len;

    return result;
}

// ============================================================================
// List Construction from Iterables
// ============================================================================

// Forward declarations for set
#define SET_OCCUPIED_FWD 1
typedef struct {
    int64_t key;
    uint8_t state;
} SetEntryFwd;
typedef struct {
    int64_t len;
    int64_t capacity;
    SetEntryFwd* entries;
} PySetFwd;

// Forward declarations for dict (int keys)
#define DICT_OCCUPIED_FWD 1
typedef struct {
    int64_t key;
    int64_t value;
    uint8_t state;
} DictEntryFwd;
typedef struct {
    int64_t len;
    int64_t capacity;
    DictEntryFwd* entries;
} PyDictFwd;

// Forward declarations for str dict (string keys)
typedef struct {
    char* key;
    int64_t value;
    uint8_t state;
} StrDictEntryFwd;
typedef struct {
    int64_t len;
    int64_t capacity;
    StrDictEntryFwd* entries;
} PyStrDictFwd;

// Create list from another list (copy)
PyList* list_from_list(PyList* src) {
    if (src == NULL) return list_new();

    PyList* result = list_with_capacity(src->len);
    if (result == NULL) return NULL;

    memcpy(result->data, src->data, src->len * sizeof(int64_t));
    result->len = src->len;

    return result;
}

// Create list from string (each character becomes its ordinal value)
PyList* list_from_str(const char* s) {
    if (s == NULL) return list_new();

    size_t len = strlen(s);
    PyList* result = list_with_capacity((int64_t)len);
    if (result == NULL) return NULL;

    for (size_t i = 0; i < len; i++) {
        result->data[i] = (int64_t)(unsigned char)s[i];
    }
    result->len = (int64_t)len;

    return result;
}

// Create list from bytes (each byte becomes its value)
PyList* list_from_bytes(const char* s) {
    if (s == NULL) return list_new();

    size_t len = strlen(s);
    PyList* result = list_with_capacity((int64_t)len);
    if (result == NULL) return NULL;

    for (size_t i = 0; i < len; i++) {
        result->data[i] = (int64_t)(unsigned char)s[i];
    }
    result->len = (int64_t)len;

    return result;
}

// Comparison for sorting int64_t
static int cmp_int64_for_list(const void* a, const void* b) {
    int64_t va = *(const int64_t*)a;
    int64_t vb = *(const int64_t*)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

// Create list from set (sorted for consistent output)
PyList* list_from_set(PySetFwd* set) {
    if (set == NULL) return list_new();

    PyList* result = list_with_capacity(set->len);
    if (result == NULL) return NULL;

    int64_t j = 0;
    for (int64_t i = 0; i < set->capacity && j < set->len; i++) {
        if (set->entries[i].state == SET_OCCUPIED_FWD) {
            result->data[j++] = set->entries[i].key;
        }
    }
    result->len = j;

    // Sort for consistent output matching Python's iteration order for small ints
    qsort(result->data, result->len, sizeof(int64_t), cmp_int64_for_list);

    return result;
}

// Create list from dict (list of keys) - for int-keyed dicts
PyList* list_from_dict(PyDictFwd* dict) {
    if (dict == NULL) return list_new();

    PyList* result = list_with_capacity(dict->len);
    if (result == NULL) return NULL;

    int64_t j = 0;
    for (int64_t i = 0; i < dict->capacity && j < dict->len; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED_FWD) {
            result->data[j++] = dict->entries[i].key;
        }
    }
    result->len = j;

    return result;
}

// Create string list from str dict (list of string keys)
PyStrList* str_list_from_str_dict(PyStrDictFwd* dict) {
    if (dict == NULL) return str_list_new();

    PyStrList* result = str_list_alloc(dict->len);
    if (result == NULL) return NULL;

    int64_t j = 0;
    for (int64_t i = 0; i < dict->capacity && j < dict->len; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED_FWD) {
            result->data[j] = strdup(dict->entries[i].key);
            if (result->data[j] == NULL) {
                // Clean up on failure
                for (int64_t k = 0; k < j; k++) free(result->data[k]);
                free(result->data);
                free(result);
                return NULL;
            }
            j++;
        }
    }
    result->len = j;

    return result;
}

// ============================================================================
// Type-Specific List Operations
// ============================================================================

// --- String List Operations (PyStrList) ---

// Grow the string list's data array
static void str_list_grow(PyStrList* list, int64_t min_capacity) {
    int64_t new_capacity = list->capacity * 2;
    if (new_capacity < min_capacity) {
        new_capacity = min_capacity;
    }
    char** new_data = (char**)realloc(list->data, new_capacity * sizeof(char*));
    if (new_data == NULL) return;
    list->data = new_data;
    list->capacity = new_capacity;
}

// Create string list with capacity
PyStrList* str_list_with_capacity(int64_t capacity) {
    return str_list_alloc(capacity);
}

// Append a string to a string list
void str_list_append(PyStrList* list, const char* value) {
    if (list == NULL) return;
    if (list->len >= list->capacity) {
        str_list_grow(list, list->len + 1);
    }
    // Store a copy of the string
    list->data[list->len++] = value ? strdup(value) : NULL;
}

// Get string at index
const char* str_list_getitem(PyStrList* list, int64_t index) {
    if (list == NULL) return "";
    if (index < 0) index = list->len + index;
    if (index < 0 || index >= list->len) return "";
    return list->data[index] ? list->data[index] : "";
}

// Set string at index
void str_list_setitem(PyStrList* list, int64_t index, const char* value) {
    if (list == NULL) return;
    if (index < 0) index = list->len + index;
    if (index < 0 || index >= list->len) return;
    // Free old string
    if (list->data[index]) free(list->data[index]);
    list->data[index] = value ? strdup(value) : NULL;
}

// --- Float List Operations ---
// Note: Float lists reuse PyList but store doubles

typedef struct {
    int64_t len;
    int64_t capacity;
    double* data;
} PyFloatList;

static PyFloatList* float_list_alloc(int64_t capacity) {
    if (capacity < INITIAL_CAPACITY) {
        capacity = INITIAL_CAPACITY;
    }
    PyFloatList* list = (PyFloatList*)malloc(sizeof(PyFloatList));
    if (list == NULL) return NULL;
    list->data = (double*)malloc(capacity * sizeof(double));
    if (list->data == NULL) {
        free(list);
        return NULL;
    }
    list->len = 0;
    list->capacity = capacity;
    return list;
}

// Create a new empty float list
PyFloatList* float_list_new(void) {
    return float_list_alloc(INITIAL_CAPACITY);
}

// Create a float list with pre-allocated capacity
PyFloatList* float_list_with_capacity(int64_t capacity) {
    return float_list_alloc(capacity);
}

// Get the length of the float list
int64_t float_list_len(PyFloatList* list) {
    if (list == NULL) return 0;
    return list->len;
}

// Grow the float list's data array
static void float_list_grow(PyFloatList* list, int64_t min_capacity) {
    int64_t new_capacity = list->capacity * 2;
    if (new_capacity < min_capacity) {
        new_capacity = min_capacity;
    }
    double* new_data = (double*)realloc(list->data, new_capacity * sizeof(double));
    if (new_data == NULL) return;
    list->data = new_data;
    list->capacity = new_capacity;
}

// Append a float to the list
void float_list_append(PyFloatList* list, double value) {
    if (list == NULL) return;
    if (list->len >= list->capacity) {
        float_list_grow(list, list->len + 1);
    }
    list->data[list->len++] = value;
}

// Get float at index
double float_list_getitem(PyFloatList* list, int64_t index) {
    if (list == NULL) return 0.0;
    if (index < 0) index = list->len + index;
    if (index < 0 || index >= list->len) return 0.0;
    return list->data[index];
}

// Set float at index
void float_list_setitem(PyFloatList* list, int64_t index, double value) {
    if (list == NULL) return;
    if (index < 0) index = list->len + index;
    if (index < 0 || index >= list->len) return;
    list->data[index] = value;
}

// Helper for qsort - compare doubles
static int cmp_double(const void* a, const void* b) {
    double va = *(const double*)a;
    double vb = *(const double*)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

// Sort float list in place (ascending order)
void float_list_sort(PyFloatList* list) {
    if (list == NULL || list->len <= 1) return;
    qsort(list->data, list->len, sizeof(double), cmp_double);
}

// --- Bool List Operations ---
// Note: Bool lists reuse PyList but store int8_t (bool is i1 in LLVM)

typedef struct {
    int64_t len;
    int64_t capacity;
    int8_t* data;  // Using int8_t for boolean storage
} PyBoolList;

static PyBoolList* bool_list_alloc(int64_t capacity) {
    if (capacity < INITIAL_CAPACITY) {
        capacity = INITIAL_CAPACITY;
    }
    PyBoolList* list = (PyBoolList*)malloc(sizeof(PyBoolList));
    if (list == NULL) return NULL;
    list->data = (int8_t*)malloc(capacity * sizeof(int8_t));
    if (list->data == NULL) {
        free(list);
        return NULL;
    }
    list->len = 0;
    list->capacity = capacity;
    return list;
}

// Create a new empty bool list
PyBoolList* bool_list_new(void) {
    return bool_list_alloc(INITIAL_CAPACITY);
}

// Create a bool list with pre-allocated capacity
PyBoolList* bool_list_with_capacity(int64_t capacity) {
    return bool_list_alloc(capacity);
}

// Get the length of the bool list
int64_t bool_list_len(PyBoolList* list) {
    if (list == NULL) return 0;
    return list->len;
}

// Grow the bool list's data array
static void bool_list_grow(PyBoolList* list, int64_t min_capacity) {
    int64_t new_capacity = list->capacity * 2;
    if (new_capacity < min_capacity) {
        new_capacity = min_capacity;
    }
    int8_t* new_data = (int8_t*)realloc(list->data, new_capacity * sizeof(int8_t));
    if (new_data == NULL) return;
    list->data = new_data;
    list->capacity = new_capacity;
}

// Append a bool to the list
void bool_list_append(PyBoolList* list, int8_t value) {
    if (list == NULL) return;
    if (list->len >= list->capacity) {
        bool_list_grow(list, list->len + 1);
    }
    list->data[list->len++] = value ? 1 : 0;
}

// Get bool at index
int8_t bool_list_getitem(PyBoolList* list, int64_t index) {
    if (list == NULL) return 0;
    if (index < 0) index = list->len + index;
    if (index < 0 || index >= list->len) return 0;
    return list->data[index];
}

// Set bool at index
void bool_list_setitem(PyBoolList* list, int64_t index, int8_t value) {
    if (list == NULL) return;
    if (index < 0) index = list->len + index;
    if (index < 0 || index >= list->len) return;
    list->data[index] = value ? 1 : 0;
}

// Helper for qsort - compare int8_t (bools)
static int cmp_int8(const void* a, const void* b) {
    int8_t va = *(const int8_t*)a;
    int8_t vb = *(const int8_t*)b;
    return va - vb;
}

// Sort bool list in place (False before True)
void bool_list_sort(PyBoolList* list) {
    if (list == NULL || list->len <= 1) return;
    qsort(list->data, list->len, sizeof(int8_t), cmp_int8);
}

// Print bool list
void print_bool_list(PyBoolList* list) {
    printf("[");
    if (list != NULL) {
        for (int64_t i = 0; i < list->len; i++) {
            if (i > 0) printf(", ");
            printf("%s", list->data[i] ? "True" : "False");
        }
    }
    printf("]");
}

// ============================================================================
// Type-Specific List Methods: pop, clear, copy
// ============================================================================

// --- String List pop, clear, copy ---

// Pop string at index (default -1 for last element)
const char* str_list_pop(PyStrList* list, int64_t index) {
    if (list == NULL || list->len == 0) return "";
    if (index < 0) index = list->len + index;
    if (index < 0 || index >= list->len) return "";

    const char* result = list->data[index] ? list->data[index] : "";
    // Don't free - return the pointer, caller owns it now
    // Shift elements after index
    for (int64_t i = index; i < list->len - 1; i++) {
        list->data[i] = list->data[i + 1];
    }
    list->len--;
    return result;
}

// Clear string list
void str_list_clear(PyStrList* list) {
    if (list == NULL) return;
    // Free all strings
    for (int64_t i = 0; i < list->len; i++) {
        if (list->data[i]) free(list->data[i]);
    }
    list->len = 0;
}

// Copy string list
PyStrList* str_list_copy(PyStrList* list) {
    if (list == NULL) return str_list_new();
    PyStrList* copy = str_list_alloc(list->capacity);
    if (copy == NULL) return NULL;
    for (int64_t i = 0; i < list->len; i++) {
        copy->data[i] = list->data[i] ? strdup(list->data[i]) : NULL;
    }
    copy->len = list->len;
    return copy;
}

// --- Float List pop, clear, copy ---

// Pop float at index (default -1 for last element)
double float_list_pop(PyFloatList* list, int64_t index) {
    if (list == NULL || list->len == 0) return 0.0;
    if (index < 0) index = list->len + index;
    if (index < 0 || index >= list->len) return 0.0;

    double result = list->data[index];
    // Shift elements after index
    for (int64_t i = index; i < list->len - 1; i++) {
        list->data[i] = list->data[i + 1];
    }
    list->len--;
    return result;
}

// Clear float list
void float_list_clear(PyFloatList* list) {
    if (list == NULL) return;
    list->len = 0;
}

// Copy float list
PyFloatList* float_list_copy(PyFloatList* list) {
    if (list == NULL) return float_list_new();
    PyFloatList* copy = float_list_alloc(list->capacity);
    if (copy == NULL) return NULL;
    memcpy(copy->data, list->data, list->len * sizeof(double));
    copy->len = list->len;
    return copy;
}

// --- Bool List pop, clear, copy ---

// Pop bool at index (default -1 for last element)
int8_t bool_list_pop(PyBoolList* list, int64_t index) {
    if (list == NULL || list->len == 0) return 0;
    if (index < 0) index = list->len + index;
    if (index < 0 || index >= list->len) return 0;

    int8_t result = list->data[index];
    // Shift elements after index
    for (int64_t i = index; i < list->len - 1; i++) {
        list->data[i] = list->data[i + 1];
    }
    list->len--;
    return result;
}

// Clear bool list
void bool_list_clear(PyBoolList* list) {
    if (list == NULL) return;
    list->len = 0;
}

// Copy bool list
PyBoolList* bool_list_copy(PyBoolList* list) {
    if (list == NULL) return bool_list_new();
    PyBoolList* copy = bool_list_alloc(list->capacity);
    if (copy == NULL) return NULL;
    memcpy(copy->data, list->data, list->len * sizeof(int8_t));
    copy->len = list->len;
    return copy;
}

// ============================================================================
// any() and all() builtins for lists
// ============================================================================

// any(list[int]) - returns true if any element is non-zero
int64_t list_any(PyList* list) {
    if (list == NULL || list->len == 0) return 0;
    for (int64_t i = 0; i < list->len; i++) {
        if (list->data[i] != 0) return 1;
    }
    return 0;
}

// all(list[int]) - returns true if all elements are non-zero
int64_t list_all(PyList* list) {
    if (list == NULL) return 1;  // Empty list returns True for all()
    if (list->len == 0) return 1;
    for (int64_t i = 0; i < list->len; i++) {
        if (list->data[i] == 0) return 0;
    }
    return 1;
}

// any(list[float]) - returns true if any element is non-zero
int64_t float_list_any(PyFloatList* list) {
    if (list == NULL || list->len == 0) return 0;
    for (int64_t i = 0; i < list->len; i++) {
        if (list->data[i] != 0.0) return 1;
    }
    return 0;
}

// all(list[float]) - returns true if all elements are non-zero
int64_t float_list_all(PyFloatList* list) {
    if (list == NULL) return 1;
    if (list->len == 0) return 1;
    for (int64_t i = 0; i < list->len; i++) {
        if (list->data[i] == 0.0) return 0;
    }
    return 1;
}

// any(list[bool]) - returns true if any element is True
int64_t bool_list_any(PyBoolList* list) {
    if (list == NULL || list->len == 0) return 0;
    for (int64_t i = 0; i < list->len; i++) {
        if (list->data[i]) return 1;
    }
    return 0;
}

// all(list[bool]) - returns true if all elements are True
int64_t bool_list_all(PyBoolList* list) {
    if (list == NULL) return 1;
    if (list->len == 0) return 1;
    for (int64_t i = 0; i < list->len; i++) {
        if (!list->data[i]) return 0;
    }
    return 1;
}

// any(list[str]) - returns true if any string is non-empty
int64_t str_list_any(PyStrList* list) {
    if (list == NULL || list->len == 0) return 0;
    for (int64_t i = 0; i < list->len; i++) {
        if (list->data[i] != NULL && list->data[i][0] != '\0') return 1;
    }
    return 0;
}

// all(list[str]) - returns true if all strings are non-empty
int64_t str_list_all(PyStrList* list) {
    if (list == NULL) return 1;
    if (list->len == 0) return 1;
    for (int64_t i = 0; i < list->len; i++) {
        if (list->data[i] == NULL || list->data[i][0] == '\0') return 0;
    }
    return 1;
}

// ============================================================================
// enumerate(), zip(), filter(), iter() builtins
// These return placeholder iterator objects for now
// ============================================================================

// Enumerate iterator structure
typedef struct {
    void* source;   // Pointer to source iterable
    int64_t index;  // Current index to return (may start at non-zero)
    int64_t pos;    // Current position in the iterable (always starts at 0)
} EnumerateIter;

// Create enumerate iterator from list with start index
EnumerateIter* enumerate_list(PyList* list, int64_t start) {
    EnumerateIter* iter = (EnumerateIter*)malloc(sizeof(EnumerateIter));
    if (iter == NULL) return NULL;
    iter->source = list;
    iter->index = start;  // Returned index starts at start
    iter->pos = 0;        // Position in list starts at 0
    return iter;
}

// Create enumerate iterator from string with start index
EnumerateIter* enumerate_str(const char* s, int64_t start) {
    EnumerateIter* iter = (EnumerateIter*)malloc(sizeof(EnumerateIter));
    if (iter == NULL) return NULL;
    iter->source = (void*)s;
    iter->index = start;  // Returned index starts at start
    iter->pos = 0;        // Position in string starts at 0
    return iter;
}

// Create enumerate iterator from bytes with start index
EnumerateIter* enumerate_bytes(const char* s, int64_t start) {
    return enumerate_str(s, start);  // Same as string
}

// Get next (index, value) pair from enumerate iterator over list
// Returns 1 if successful, 0 if exhausted
// out_index receives the index, out_value receives the list element
int64_t enumerate_list_next(EnumerateIter* iter, int64_t* out_index, int64_t* out_value) {
    if (iter == NULL) return 0;

    PyList* list = (PyList*)iter->source;
    if (list == NULL || iter->pos >= list->len) return 0;

    *out_index = iter->index;          // Return the counter (may be offset by start)
    *out_value = list->data[iter->pos]; // Access data at actual position
    iter->index++;
    iter->pos++;
    return 1;
}

// Get next (index, char) pair from enumerate iterator over string
// Returns 1 if successful, 0 if exhausted
// out_index receives the index, out_value receives single-char string (pointer)
int64_t enumerate_str_next(EnumerateIter* iter, int64_t* out_index, int64_t* out_value) {
    if (iter == NULL) return 0;

    const char* s = (const char*)iter->source;
    if (s == NULL || s[iter->pos] == '\0') return 0;

    *out_index = iter->index;                       // Return the counter (may be offset by start)
    *out_value = (int64_t)local_str_char_at(s, iter->pos);  // Access char at actual position
    iter->index++;
    iter->pos++;
    return 1;
}

// Get next (index, byte) pair from enumerate iterator over bytes
// Returns 1 if successful, 0 if exhausted
// out_index receives the index, out_value receives byte value as int
int64_t enumerate_bytes_next(EnumerateIter* iter, int64_t* out_index, int64_t* out_value) {
    if (iter == NULL) return 0;

    const char* s = (const char*)iter->source;
    if (s == NULL || s[iter->pos] == '\0') return 0;

    *out_index = iter->index;                         // Return the counter (may be offset by start)
    *out_value = (int64_t)(unsigned char)s[iter->pos]; // Access byte at actual position
    iter->index++;
    iter->pos++;
    return 1;
}

// Free enumerate iterator
void enumerate_free(EnumerateIter* iter) {
    if (iter) free(iter);
}

// Zip iterator structure
typedef struct {
    void* source1;
    void* source2;
    void* source3;  // For 3-way zip
    int64_t index;
} ZipIter;

// Create zip iterator (single iterable - identity zip)
ZipIter* zip_single(void* iterable) {
    ZipIter* iter = (ZipIter*)malloc(sizeof(ZipIter));
    if (iter == NULL) return NULL;
    iter->source1 = iterable;
    iter->source2 = NULL;
    iter->source3 = NULL;
    iter->index = 0;
    return iter;
}

// Create zip iterator (two iterables)
ZipIter* zip_two(void* iter1, void* iter2) {
    ZipIter* iter = (ZipIter*)malloc(sizeof(ZipIter));
    if (iter == NULL) return NULL;
    iter->source1 = iter1;
    iter->source2 = iter2;
    iter->source3 = NULL;
    iter->index = 0;
    return iter;
}

// Create zip iterator (three iterables)
ZipIter* zip_three(void* iter1, void* iter2, void* iter3) {
    ZipIter* iter = (ZipIter*)malloc(sizeof(ZipIter));
    if (iter == NULL) return NULL;
    iter->source1 = iter1;
    iter->source2 = iter2;
    iter->source3 = iter3;
    iter->index = 0;
    return iter;
}

// Get next pair from zip iterator over two lists
// Returns 1 if successful, 0 if either list is exhausted
int64_t zip_two_list_next(ZipIter* iter, int64_t* out_val1, int64_t* out_val2) {
    if (iter == NULL) return 0;

    PyList* list1 = (PyList*)iter->source1;
    PyList* list2 = (PyList*)iter->source2;

    if (list1 == NULL || list2 == NULL) return 0;
    if (iter->index >= list1->len || iter->index >= list2->len) return 0;

    *out_val1 = list1->data[iter->index];
    *out_val2 = list2->data[iter->index];
    iter->index++;
    return 1;
}

// Get next pair from zip iterator over two strings
// Returns 1 if successful, 0 if either string is exhausted
int64_t zip_two_str_next(ZipIter* iter, int64_t* out_val1, int64_t* out_val2) {
    if (iter == NULL) return 0;

    const char* str1 = (const char*)iter->source1;
    const char* str2 = (const char*)iter->source2;

    if (str1 == NULL || str2 == NULL) return 0;
    if (str1[iter->index] == '\0' || str2[iter->index] == '\0') return 0;

    // Return single-char strings
    *out_val1 = (int64_t)local_str_char_at(str1, iter->index);
    *out_val2 = (int64_t)local_str_char_at(str2, iter->index);
    iter->index++;
    return 1;
}

// Get next triple from zip iterator over three lists
// Returns 1 if successful, 0 if any list is exhausted
int64_t zip_three_list_next(ZipIter* iter, int64_t* out_val1, int64_t* out_val2, int64_t* out_val3) {
    if (iter == NULL) return 0;

    PyList* list1 = (PyList*)iter->source1;
    PyList* list2 = (PyList*)iter->source2;
    PyList* list3 = (PyList*)iter->source3;

    if (list1 == NULL || list2 == NULL || list3 == NULL) return 0;
    if (iter->index >= list1->len || iter->index >= list2->len || iter->index >= list3->len) return 0;

    *out_val1 = list1->data[iter->index];
    *out_val2 = list2->data[iter->index];
    *out_val3 = list3->data[iter->index];
    iter->index++;
    return 1;
}

// Free zip iterator
void zip_free(ZipIter* iter) {
    if (iter) free(iter);
}

// Filter iterator structure
typedef struct {
    void* source;
    void* func;  // NULL means filter by truthiness
} FilterIter;

// Create filter iterator with None (filter by truthiness)
FilterIter* filter_none(void* iterable) {
    FilterIter* iter = (FilterIter*)malloc(sizeof(FilterIter));
    if (iter == NULL) return NULL;
    iter->source = iterable;
    iter->func = NULL;
    return iter;
}

// Generic iterator structure
typedef struct {
    void* source;
    int64_t index;
} GenericIter;

// Create generic iterator from list
GenericIter* iter_list(PyList* list) {
    GenericIter* iter = (GenericIter*)malloc(sizeof(GenericIter));
    if (iter == NULL) return NULL;
    iter->source = list;
    iter->index = 0;
    return iter;
}

// Create generic iterator from string
GenericIter* iter_str(const char* s) {
    GenericIter* iter = (GenericIter*)malloc(sizeof(GenericIter));
    if (iter == NULL) return NULL;
    iter->source = (void*)s;
    iter->index = 0;
    return iter;
}

// Create generic iterator from bytes
GenericIter* iter_bytes(const char* s) {
    return iter_str(s);
}

// ============================================================================
// next() builtin - get next value from iterator
// ============================================================================

// Get next value from list iterator with default
// If iterator is exhausted, returns default_value
// Sets *exhausted to 1 if exhausted, 0 otherwise (pass NULL to ignore)
int64_t iter_next_list(GenericIter* iter, int64_t default_value, int8_t* exhausted) {
    if (exhausted) *exhausted = 0;
    if (iter == NULL) {
        if (exhausted) *exhausted = 1;
        return default_value;
    }

    PyList* list = (PyList*)iter->source;
    if (list == NULL || iter->index >= list->len) {
        if (exhausted) *exhausted = 1;
        return default_value;
    }

    return list->data[iter->index++];
}

// Get next character from string iterator (returns single-char string pointer)
// Returns default_value cast to char* if exhausted (caller should check exhausted flag)
char* iter_next_str(GenericIter* iter, char* default_value, int8_t* exhausted) {
    if (exhausted) *exhausted = 0;
    if (iter == NULL) {
        if (exhausted) *exhausted = 1;
        return default_value;
    }

    const char* s = (const char*)iter->source;
    if (s == NULL || s[iter->index] == '\0') {
        if (exhausted) *exhausted = 1;
        return default_value;
    }

    // Create a single-character string and return it
    char* result = local_str_char_at(s, iter->index++);
    return result;
}

// Get next character from string iterator (returns ordinal value)
int64_t iter_next_str_ord(GenericIter* iter, int64_t default_value, int8_t* exhausted) {
    if (exhausted) *exhausted = 0;
    if (iter == NULL) {
        if (exhausted) *exhausted = 1;
        return default_value;
    }

    const char* s = (const char*)iter->source;
    if (s == NULL || s[iter->index] == '\0') {
        if (exhausted) *exhausted = 1;
        return default_value;
    }

    return (int64_t)(unsigned char)s[iter->index++];
}

// Get next byte from bytes iterator (returns ordinal value)
int64_t iter_next_bytes(GenericIter* iter, int64_t default_value, int8_t* exhausted) {
    return iter_next_str_ord(iter, default_value, exhausted);
}

// Check if iterator has more values (without consuming)
int64_t iter_has_next_list(GenericIter* iter) {
    if (iter == NULL) return 0;
    PyList* list = (PyList*)iter->source;
    if (list == NULL) return 0;
    return iter->index < list->len ? 1 : 0;
}

int64_t iter_has_next_str(GenericIter* iter) {
    if (iter == NULL) return 0;
    const char* s = (const char*)iter->source;
    if (s == NULL) return 0;
    return s[iter->index] != '\0' ? 1 : 0;
}

// ============================================================================
// id() builtin - returns the memory address of an object as an integer
// ============================================================================

int64_t id_ptr(void* ptr) {
    return (int64_t)(uintptr_t)ptr;
}

int64_t id_int(int64_t value) {
    // For immediate values, we return a hash-like value
    // Python returns unique ids for small ints, but we can just use the value
    return value;
}

int64_t id_float(double value) {
    // Return a hash of the float bits
    union { double d; int64_t i; } u;
    u.d = value;
    return u.i;
}

// ============================================================================
// str.split() - returns list of strings split by separator
// ============================================================================

// Split string by separator, returns a new PyStrList
PyStrList* str_split(const char* s, const char* sep) {
    if (s == NULL) return str_list_new();

    PyStrList* result = str_list_new();
    if (result == NULL) return NULL;

    size_t seplen = sep ? strlen(sep) : 0;

    // If no separator or empty separator, split on whitespace
    if (sep == NULL || seplen == 0) {
        // Split on whitespace
        const char* p = s;
        while (*p) {
            // Skip leading whitespace
            while (*p && isspace((unsigned char)*p)) p++;
            if (!*p) break;

            // Find end of word
            const char* start = p;
            while (*p && !isspace((unsigned char)*p)) p++;

            // Create substring
            size_t len = p - start;
            char* word = (char*)malloc(len + 1);
            if (word) {
                memcpy(word, start, len);
                word[len] = '\0';
                str_list_append(result, word);
                free(word);
            }
        }
        return result;
    }

    // Split on separator
    const char* p = s;
    while (*p) {
        const char* found = strstr(p, sep);
        if (found == NULL) {
            // Append remaining string
            str_list_append(result, p);
            break;
        }

        // Append substring before separator
        size_t len = found - p;
        char* part = (char*)malloc(len + 1);
        if (part) {
            memcpy(part, p, len);
            part[len] = '\0';
            str_list_append(result, part);
            free(part);
        }

        p = found + seplen;
    }

    return result;
}

// Join a string list with separator - "sep".join(list[str])
char* str_list_join(const char* sep, PyStrList* list) {
    if (list == NULL || list->len == 0) return strdup("");
    if (list->len == 1) return strdup(list->data[0] ? list->data[0] : "");

    size_t seplen = sep ? strlen(sep) : 0;

    // Calculate total length
    size_t total = 0;
    for (int64_t i = 0; i < list->len; i++) {
        if (list->data[i]) total += strlen(list->data[i]);
    }
    total += seplen * (list->len - 1);

    char* result = (char*)malloc(total + 1);
    if (result == NULL) return NULL;

    char* dst = result;
    for (int64_t i = 0; i < list->len; i++) {
        if (i > 0 && sep) {
            memcpy(dst, sep, seplen);
            dst += seplen;
        }
        if (list->data[i]) {
            size_t partlen = strlen(list->data[i]);
            memcpy(dst, list->data[i], partlen);
            dst += partlen;
        }
    }
    *dst = '\0';

    return result;
}

// ============================================================================
// repr() for containers - returns simplified string representations
// ============================================================================

// repr(list) - returns string like "[1, 2, 3]"
char* repr_list(PyList* list) {
    if (list == NULL || list->len == 0) return strdup("[]");

    // Estimate size: each element max 20 chars + ", " + brackets
    size_t est_size = list->len * 22 + 3;
    char* result = (char*)malloc(est_size);
    if (result == NULL) return NULL;

    strcpy(result, "[");
    char buf[32];
    for (int64_t i = 0; i < list->len; i++) {
        if (i > 0) strcat(result, ", ");
        snprintf(buf, sizeof(buf), "%lld", (long long)list->data[i]);
        strcat(result, buf);
    }
    strcat(result, "]");
    return result;
}
