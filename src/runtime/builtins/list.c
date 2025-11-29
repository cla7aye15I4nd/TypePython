// TypePython Runtime Library - List Module
// Dynamic array implementation for list type
//
// Memory layout: PyList struct with flexible array member
// Assignment creates a copy (not alias) for safety with realloc

#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

// ============================================================================
// List Data Structure
// ============================================================================

typedef struct {
    int64_t len;       // Current number of elements
    int64_t capacity;  // Allocated capacity
    int64_t* data;     // Pointer to element array (separate allocation)
} PyList;

#define INITIAL_CAPACITY 8

// ============================================================================
// Internal Helper Functions
// ============================================================================

static PyList* list_alloc(int64_t capacity) {
    if (capacity < INITIAL_CAPACITY) {
        capacity = INITIAL_CAPACITY;
    }
    PyList* list = (PyList*)malloc(sizeof(PyList));
    if (list == NULL) return NULL;
    list->data = (int64_t*)malloc(capacity * sizeof(int64_t));
    if (list->data == NULL) {
        free(list);
        return NULL;
    }
    list->len = 0;
    list->capacity = capacity;
    return list;
}

// Grow the list's data array (PyList struct stays at same address)
static void list_grow(PyList* list, int64_t min_capacity) {
    int64_t new_capacity = list->capacity * 2;
    if (new_capacity < min_capacity) {
        new_capacity = min_capacity;
    }
    int64_t* new_data = (int64_t*)realloc(list->data, new_capacity * sizeof(int64_t));
    if (new_data == NULL) return;  // Keep old data on failure
    list->data = new_data;
    list->capacity = new_capacity;
}

// Normalize negative index to positive
static int64_t normalize_index(int64_t index, int64_t len) {
    if (index < 0) {
        index = len + index;
    }
    return index;
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
    index = normalize_index(index, list->len);
    if (index < 0 || index >= list->len) {
        // TODO: Error handling - for now return 0
        return 0;
    }
    return list->data[index];
}

// Set item at index
void list_setitem(PyList* list, int64_t index, int64_t value) {
    if (list == NULL) return;
    index = normalize_index(index, list->len);
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

    index = normalize_index(index, list->len);
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

    index = normalize_index(index, list->len);
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
            printf("%g", data[i]);
        }
    }
    printf("]");
}
