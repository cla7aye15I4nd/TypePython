// TypePython Runtime Library - Set Module
// Hash set implementation for set type
//
// Uses open addressing with linear probing (similar to dict but keys only)

#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#include "../common.h"

// ============================================================================
// Set Data Structure
// ============================================================================

#define SET_EMPTY    0
#define SET_OCCUPIED 1
#define SET_DELETED  2

typedef struct {
    int64_t key;
    uint8_t state;
} SetEntry;

typedef struct {
    int64_t len;       // Number of items
    int64_t capacity;  // Table size
    SetEntry* entries;
} PySet;

#define INITIAL_CAPACITY 16
#define LOAD_FACTOR 0.75

// ============================================================================
// Internal Helper Functions
// ============================================================================

static int64_t find_slot(PySet* set, int64_t key, int for_insert) {
    uint64_t hash = tpy_hash_int(key);
    int64_t mask = set->capacity - 1;
    int64_t index = hash & mask;
    int64_t first_deleted = -1;

    for (int64_t i = 0; i < set->capacity; i++) {
        SetEntry* entry = &set->entries[index];
        if (entry->state == SET_EMPTY) {
            return for_insert ? (first_deleted >= 0 ? first_deleted : index) : -1;
        }
        if (entry->state == SET_DELETED) {
            if (first_deleted < 0) first_deleted = index;
        } else if (entry->key == key) {
            return index;
        }
        index = (index + 1) & mask;
    }
    return for_insert ? first_deleted : -1;
}

static PySet* set_resize(PySet* set, int64_t new_capacity) {
    SetEntry* old_entries = set->entries;
    int64_t old_capacity = set->capacity;

    set->entries = (SetEntry*)calloc(new_capacity, sizeof(SetEntry));
    if (set->entries == NULL) {
        set->entries = old_entries;
        return NULL;
    }
    set->capacity = new_capacity;
    set->len = 0;

    // Rehash all entries
    for (int64_t i = 0; i < old_capacity; i++) {
        if (old_entries[i].state == SET_OCCUPIED) {
            int64_t slot = find_slot(set, old_entries[i].key, 1);
            set->entries[slot].key = old_entries[i].key;
            set->entries[slot].state = SET_OCCUPIED;
            set->len++;
        }
    }

    free(old_entries);
    return set;
}

// ============================================================================
// Set Core Functions
// ============================================================================

// Create a new empty set
PySet* set_new(void) {
    PySet* set = (PySet*)malloc(sizeof(PySet));
    if (set == NULL) return NULL;
    set->len = 0;
    set->capacity = INITIAL_CAPACITY;
    set->entries = (SetEntry*)calloc(INITIAL_CAPACITY, sizeof(SetEntry));
    if (set->entries == NULL) {
        free(set);
        return NULL;
    }
    return set;
}

// Get the length of the set
int64_t set_len(PySet* set) {
    if (set == NULL) return 0;
    return set->len;
}

// Add an item to the set
void set_add(PySet* set, int64_t key) {
    if (set == NULL) return;

    // Resize if needed
    if ((set->len + 1) > (int64_t)(set->capacity * LOAD_FACTOR)) {
        if (set_resize(set, set->capacity * 2) == NULL) return;
    }

    int64_t slot = find_slot(set, key, 1);
    if (slot < 0) return;

    if (set->entries[slot].state != SET_OCCUPIED) {
        set->entries[slot].key = key;
        set->entries[slot].state = SET_OCCUPIED;
        set->len++;
    }
}

// Remove an item (error if missing - we silently ignore for now)
void set_remove(PySet* set, int64_t key) {
    if (set == NULL) return;
    int64_t slot = find_slot(set, key, 0);
    if (slot < 0) return;
    set->entries[slot].state = SET_DELETED;
    set->len--;
}

// Discard an item (no error if missing)
void set_discard(PySet* set, int64_t key) {
    set_remove(set, key);
}

// Check if key exists
int64_t set_contains(PySet* set, int64_t key) {
    if (set == NULL) return 0;
    return find_slot(set, key, 0) >= 0 ? 1 : 0;
}

// Check if float key exists (converts float to int for comparison)
// Python: 1.0 in {1, 2, 3} == True because 1.0 == 1
int64_t set_contains_float(PySet* set, double key) {
    if (set == NULL) return 0;
    // Convert float to int and search for that key
    int64_t int_key = (int64_t)key;
    // Only match if conversion is lossless
    if ((double)int_key == key) {
        return find_slot(set, int_key, 0) >= 0 ? 1 : 0;
    }
    return 0;
}

// Clear all items
void set_clear(PySet* set) {
    if (set == NULL) return;
    memset(set->entries, 0, set->capacity * sizeof(SetEntry));
    set->len = 0;
}

// Create a shallow copy
PySet* set_copy(PySet* set) {
    if (set == NULL) return set_new();

    PySet* copy = (PySet*)malloc(sizeof(PySet));
    if (copy == NULL) return NULL;

    copy->len = set->len;
    copy->capacity = set->capacity;
    copy->entries = (SetEntry*)malloc(set->capacity * sizeof(SetEntry));
    if (copy->entries == NULL) {
        free(copy);
        return NULL;
    }
    memcpy(copy->entries, set->entries, set->capacity * sizeof(SetEntry));
    return copy;
}

// ============================================================================
// Set Operations
// ============================================================================

// Union: {1, 2} | {2, 3} = {1, 2, 3}
PySet* set_union(PySet* set1, PySet* set2) {
    PySet* result = set_copy(set1);
    if (result == NULL) return NULL;

    if (set2 != NULL) {
        for (int64_t i = 0; i < set2->capacity; i++) {
            if (set2->entries[i].state == SET_OCCUPIED) {
                set_add(result, set2->entries[i].key);
            }
        }
    }
    return result;
}

// Intersection: {1, 2, 3} & {2, 3, 4} = {2, 3}
PySet* set_intersection(PySet* set1, PySet* set2) {
    PySet* result = set_new();
    if (result == NULL) return NULL;
    if (set1 == NULL || set2 == NULL) return result;

    for (int64_t i = 0; i < set1->capacity; i++) {
        if (set1->entries[i].state == SET_OCCUPIED) {
            if (set_contains(set2, set1->entries[i].key)) {
                set_add(result, set1->entries[i].key);
            }
        }
    }
    return result;
}

// Difference: {1, 2, 3} - {2} = {1, 3}
PySet* set_difference(PySet* set1, PySet* set2) {
    PySet* result = set_new();
    if (result == NULL) return NULL;
    if (set1 == NULL) return result;

    for (int64_t i = 0; i < set1->capacity; i++) {
        if (set1->entries[i].state == SET_OCCUPIED) {
            if (set2 == NULL || !set_contains(set2, set1->entries[i].key)) {
                set_add(result, set1->entries[i].key);
            }
        }
    }
    return result;
}

// Symmetric difference: {1, 2, 3} ^ {2, 3, 4} = {1, 4}
PySet* set_symmetric_difference(PySet* set1, PySet* set2) {
    PySet* result = set_new();
    if (result == NULL) return NULL;

    // Add items in set1 but not in set2
    if (set1 != NULL) {
        for (int64_t i = 0; i < set1->capacity; i++) {
            if (set1->entries[i].state == SET_OCCUPIED) {
                if (set2 == NULL || !set_contains(set2, set1->entries[i].key)) {
                    set_add(result, set1->entries[i].key);
                }
            }
        }
    }

    // Add items in set2 but not in set1
    if (set2 != NULL) {
        for (int64_t i = 0; i < set2->capacity; i++) {
            if (set2->entries[i].state == SET_OCCUPIED) {
                if (set1 == NULL || !set_contains(set1, set2->entries[i].key)) {
                    set_add(result, set2->entries[i].key);
                }
            }
        }
    }

    return result;
}

// Check if set1 is a subset of set2
int64_t set_issubset(PySet* set1, PySet* set2) {
    if (set1 == NULL || set1->len == 0) return 1;
    if (set2 == NULL) return 0;

    for (int64_t i = 0; i < set1->capacity; i++) {
        if (set1->entries[i].state == SET_OCCUPIED) {
            if (!set_contains(set2, set1->entries[i].key)) {
                return 0;
            }
        }
    }
    return 1;
}

// Check if set1 is a superset of set2
int64_t set_issuperset(PySet* set1, PySet* set2) {
    return set_issubset(set2, set1);
}

// Check if set1 is a proper subset of set2 (subset but not equal)
int64_t set_is_proper_subset(PySet* set1, PySet* set2) {
    int64_t len1 = set1 ? set1->len : 0;
    int64_t len2 = set2 ? set2->len : 0;
    if (len1 >= len2) return 0;
    return set_issubset(set1, set2);
}

// Check if set1 is a proper superset of set2
int64_t set_is_proper_superset(PySet* set1, PySet* set2) {
    return set_is_proper_subset(set2, set1);
}

// Check if two sets are disjoint (no common elements)
int64_t set_isdisjoint(PySet* set1, PySet* set2) {
    if (set1 == NULL || set2 == NULL) return 1;
    if (set1->len == 0 || set2->len == 0) return 1;

    // Iterate over smaller set
    PySet* smaller = set1->len < set2->len ? set1 : set2;
    PySet* larger = set1->len < set2->len ? set2 : set1;

    for (int64_t i = 0; i < smaller->capacity; i++) {
        if (smaller->entries[i].state == SET_OCCUPIED) {
            if (set_contains(larger, smaller->entries[i].key)) {
                return 0;
            }
        }
    }
    return 1;
}

// Check equality
int64_t set_eq(PySet* set1, PySet* set2) {
    int64_t len1 = set1 ? set1->len : 0;
    int64_t len2 = set2 ? set2->len : 0;

    if (len1 != len2) return 0;
    if (len1 == 0) return 1;

    return set_issubset(set1, set2);
}

// Compare two sets for min/max operations
// Returns: -1 if set1 is proper subset of set2
//           1 if set1 is proper superset of set2
//           0 if incomparable or equal
int64_t set_cmp(PySet* set1, PySet* set2) {
    if (set_is_proper_subset(set1, set2)) return -1;
    if (set_is_proper_superset(set1, set2)) return 1;
    return 0;
}

// ============================================================================
// In-place Update Operations
// ============================================================================

// Pop and return an arbitrary element (returns 0 if empty, caller should check len)
int64_t set_pop(PySet* set) {
    if (set == NULL || set->len == 0) return 0;

    // Find first occupied slot
    for (int64_t i = 0; i < set->capacity; i++) {
        if (set->entries[i].state == SET_OCCUPIED) {
            int64_t key = set->entries[i].key;
            set->entries[i].state = SET_DELETED;
            set->len--;
            return key;
        }
    }
    return 0;
}

// Update set with another set (in-place union)
void set_update(PySet* set, PySet* other) {
    if (set == NULL || other == NULL) return;

    for (int64_t i = 0; i < other->capacity; i++) {
        if (other->entries[i].state == SET_OCCUPIED) {
            set_add(set, other->entries[i].key);
        }
    }
}

// Update set with difference (in-place)
void set_difference_update(PySet* set, PySet* other) {
    if (set == NULL || other == NULL) return;

    for (int64_t i = 0; i < set->capacity; i++) {
        if (set->entries[i].state == SET_OCCUPIED) {
            if (set_contains(other, set->entries[i].key)) {
                set->entries[i].state = SET_DELETED;
                set->len--;
            }
        }
    }
}

// Update set with intersection (in-place)
void set_intersection_update(PySet* set, PySet* other) {
    if (set == NULL) return;
    if (other == NULL) {
        set_clear(set);
        return;
    }

    for (int64_t i = 0; i < set->capacity; i++) {
        if (set->entries[i].state == SET_OCCUPIED) {
            if (!set_contains(other, set->entries[i].key)) {
                set->entries[i].state = SET_DELETED;
                set->len--;
            }
        }
    }
}

// Update set with symmetric difference (in-place)
void set_symmetric_difference_update(PySet* set, PySet* other) {
    if (set == NULL || other == NULL) return;

    // For each element in other:
    // - If in set, remove it
    // - If not in set, add it
    for (int64_t i = 0; i < other->capacity; i++) {
        if (other->entries[i].state == SET_OCCUPIED) {
            int64_t key = other->entries[i].key;
            int64_t slot = find_slot(set, key, 0);
            if (slot >= 0) {
                // Key exists in set, remove it
                set->entries[slot].state = SET_DELETED;
                set->len--;
            } else {
                // Key doesn't exist, add it
                set_add(set, key);
            }
        }
    }
}

// Find the maximum element in the set
int64_t set_max(PySet* set) {
    if (set == NULL || set->len == 0) return 0;
    int64_t max_val = 0;
    int first = 1;
    for (int64_t i = 0; i < set->capacity; i++) {
        if (set->entries[i].state == SET_OCCUPIED) {
            if (first || set->entries[i].key > max_val) {
                max_val = set->entries[i].key;
                first = 0;
            }
        }
    }
    return max_val;
}

// Find the minimum element in the set
int64_t set_min(PySet* set) {
    if (set == NULL || set->len == 0) return 0;
    int64_t min_val = 0;
    int first = 1;
    for (int64_t i = 0; i < set->capacity; i++) {
        if (set->entries[i].state == SET_OCCUPIED) {
            if (first || set->entries[i].key < min_val) {
                min_val = set->entries[i].key;
                first = 0;
            }
        }
    }
    return min_val;
}

// ============================================================================
// Print Support
// ============================================================================

// Comparison function for qsort
static int cmp_int64(const void* a, const void* b) {
    int64_t va = *(const int64_t*)a;
    int64_t vb = *(const int64_t*)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

void print_set(PySet* set) {
    if (set != NULL && set->len == 0) {
        printf("set()");
        return;
    }
    printf("{");
    if (set != NULL && set->len > 0) {
        // Collect elements and sort them for consistent output
        int64_t* elements = (int64_t*)malloc(set->len * sizeof(int64_t));
        int64_t count = 0;
        for (int64_t i = 0; i < set->capacity; i++) {
            if (set->entries[i].state == SET_OCCUPIED) {
                elements[count++] = set->entries[i].key;
            }
        }
        qsort(elements, count, sizeof(int64_t), cmp_int64);
        for (int64_t i = 0; i < count; i++) {
            if (i > 0) printf(", ");
            printf("%ld", elements[i]);
        }
        free(elements);
    }
    printf("}");
}

// ============================================================================
// String Set (PyStrSet) - stores string pointers instead of int64_t
// ============================================================================

typedef struct {
    char* key;
    uint8_t state;
} StrSetEntry;

typedef struct {
    int64_t len;
    int64_t capacity;
    StrSetEntry* entries;
} PyStrSet;

// Simple string hash function (djb2)
static uint64_t hash_str(const char* str) {
    uint64_t hash = 5381;
    int c;
    while ((c = *str++)) {
        hash = ((hash << 5) + hash) + c;
    }
    return hash;
}

static int64_t str_set_find_slot(PyStrSet* set, const char* key, int for_insert) {
    uint64_t h = hash_str(key);
    int64_t mask = set->capacity - 1;
    int64_t index = h & mask;
    int64_t first_deleted = -1;

    for (int64_t i = 0; i < set->capacity; i++) {
        StrSetEntry* entry = &set->entries[index];
        if (entry->state == SET_EMPTY) {
            return for_insert ? (first_deleted >= 0 ? first_deleted : index) : -1;
        }
        if (entry->state == SET_DELETED) {
            if (first_deleted < 0) first_deleted = index;
        } else if (strcmp(entry->key, key) == 0) {
            return index;
        }
        index = (index + 1) & mask;
    }
    return for_insert ? (first_deleted >= 0 ? first_deleted : -1) : -1;
}

static void str_set_grow(PyStrSet* set) {
    int64_t old_capacity = set->capacity;
    StrSetEntry* old_entries = set->entries;

    set->capacity *= 2;
    set->entries = (StrSetEntry*)calloc(set->capacity, sizeof(StrSetEntry));
    set->len = 0;

    for (int64_t i = 0; i < old_capacity; i++) {
        if (old_entries[i].state == SET_OCCUPIED) {
            int64_t slot = str_set_find_slot(set, old_entries[i].key, 1);
            set->entries[slot].key = old_entries[i].key;
            set->entries[slot].state = SET_OCCUPIED;
            set->len++;
        }
    }
    free(old_entries);
}

PyStrSet* str_set_new(void) {
    PyStrSet* set = (PyStrSet*)malloc(sizeof(PyStrSet));
    if (set == NULL) return NULL;
    set->capacity = INITIAL_CAPACITY;
    set->len = 0;
    set->entries = (StrSetEntry*)calloc(set->capacity, sizeof(StrSetEntry));
    if (set->entries == NULL) {
        free(set);
        return NULL;
    }
    return set;
}

void str_set_add(PyStrSet* set, const char* key) {
    if (set == NULL || key == NULL) return;

    if ((double)set->len / set->capacity >= LOAD_FACTOR) {
        str_set_grow(set);
    }

    int64_t slot = str_set_find_slot(set, key, 1);
    if (slot < 0) return;

    if (set->entries[slot].state != SET_OCCUPIED) {
        set->entries[slot].key = strdup(key);
        set->entries[slot].state = SET_OCCUPIED;
        set->len++;
    }
}

int64_t str_set_contains(PyStrSet* set, const char* key) {
    if (set == NULL || key == NULL) return 0;
    int64_t slot = str_set_find_slot(set, key, 0);
    return slot >= 0 ? 1 : 0;
}

// Create string set from a string (unique characters)
PyStrSet* str_set_from_str(const char* s) {
    PyStrSet* result = str_set_new();
    if (result == NULL || s == NULL) return result;

    size_t len = strlen(s);
    for (size_t i = 0; i < len; i++) {
        char single[2] = { s[i], '\0' };
        str_set_add(result, single);
    }
    return result;
}

// Comparison function for qsort (strings)
static int cmp_str(const void* a, const void* b) {
    return strcmp(*(const char**)a, *(const char**)b);
}

// Print string set: {'h', 'e', 'l', 'o'}
void print_str_set(PyStrSet* set) {
    if (set != NULL && set->len == 0) {
        printf("set()");
        return;
    }
    printf("{");
    if (set != NULL && set->len > 0) {
        // Collect and sort strings
        char** elements = (char**)malloc(set->len * sizeof(char*));
        int64_t count = 0;
        for (int64_t i = 0; i < set->capacity; i++) {
            if (set->entries[i].state == SET_OCCUPIED) {
                elements[count++] = set->entries[i].key;
            }
        }
        qsort(elements, count, sizeof(char*), cmp_str);
        for (int64_t i = 0; i < count; i++) {
            if (i > 0) printf(", ");
            printf("'%s'", elements[i]);
        }
        free(elements);
    }
    printf("}");
}

// --- String Set additional methods ---

// Remove a string from the set (raises error if not found - we just ignore)
void str_set_remove(PyStrSet* set, const char* key) {
    if (set == NULL || key == NULL) return;
    int64_t slot = str_set_find_slot(set, key, 0);
    if (slot >= 0) {
        free(set->entries[slot].key);
        set->entries[slot].key = NULL;
        set->entries[slot].state = SET_DELETED;
        set->len--;
    }
}

// Discard a string from the set (no error if not found)
void str_set_discard(PyStrSet* set, const char* key) {
    str_set_remove(set, key);  // Same implementation
}

// Clear all strings from the set
void str_set_clear(PyStrSet* set) {
    if (set == NULL) return;
    for (int64_t i = 0; i < set->capacity; i++) {
        if (set->entries[i].state == SET_OCCUPIED) {
            free(set->entries[i].key);
            set->entries[i].key = NULL;
            set->entries[i].state = SET_EMPTY;
        }
    }
    set->len = 0;
}

// Pop an arbitrary string from the set
const char* str_set_pop(PyStrSet* set) {
    if (set == NULL || set->len == 0) return "";
    for (int64_t i = 0; i < set->capacity; i++) {
        if (set->entries[i].state == SET_OCCUPIED) {
            const char* result = set->entries[i].key;
            set->entries[i].key = NULL;
            set->entries[i].state = SET_DELETED;
            set->len--;
            return result;
        }
    }
    return "";
}

// Copy string set
PyStrSet* str_set_copy(PyStrSet* set) {
    if (set == NULL) return str_set_new();
    PyStrSet* copy = (PyStrSet*)malloc(sizeof(PyStrSet));
    if (copy == NULL) return NULL;
    copy->capacity = set->capacity;
    copy->len = 0;
    copy->entries = (StrSetEntry*)calloc(copy->capacity, sizeof(StrSetEntry));
    if (copy->entries == NULL) {
        free(copy);
        return NULL;
    }
    for (int64_t i = 0; i < set->capacity; i++) {
        if (set->entries[i].state == SET_OCCUPIED) {
            copy->entries[i].key = strdup(set->entries[i].key);
            copy->entries[i].state = SET_OCCUPIED;
            copy->len++;
        }
    }
    return copy;
}

// ============================================================================
// Float Set (PyFloatSet)
// ============================================================================

typedef struct {
    double key;
    uint8_t state;
} FloatSetEntry;

typedef struct {
    int64_t len;
    int64_t capacity;
    FloatSetEntry* entries;
} PyFloatSet;

// Hash function for float keys (bitcast to int64_t)
static uint64_t hash_float(double key) {
    union { double d; uint64_t i; } u;
    u.d = key;
    uint64_t h = u.i;
    h ^= h >> 33;
    h *= 0xff51afd7ed558ccdULL;
    h ^= h >> 33;
    h *= 0xc4ceb9fe1a85ec53ULL;
    h ^= h >> 33;
    return h;
}

static int64_t float_set_find_slot(PyFloatSet* set, double key, int for_insert) {
    uint64_t hash = hash_float(key);
    int64_t mask = set->capacity - 1;
    int64_t index = hash & mask;
    int64_t first_deleted = -1;

    for (int64_t i = 0; i < set->capacity; i++) {
        FloatSetEntry* entry = &set->entries[index];
        if (entry->state == SET_EMPTY) {
            return for_insert ? (first_deleted >= 0 ? first_deleted : index) : -1;
        }
        if (entry->state == SET_DELETED) {
            if (first_deleted < 0) first_deleted = index;
        } else if (entry->key == key) {
            return index;
        }
        index = (index + 1) & mask;
    }
    return for_insert ? first_deleted : -1;
}

static void float_set_grow(PyFloatSet* set) {
    int64_t old_capacity = set->capacity;
    FloatSetEntry* old_entries = set->entries;
    set->capacity *= 2;
    set->entries = (FloatSetEntry*)calloc(set->capacity, sizeof(FloatSetEntry));
    set->len = 0;
    for (int64_t i = 0; i < old_capacity; i++) {
        if (old_entries[i].state == SET_OCCUPIED) {
            int64_t slot = float_set_find_slot(set, old_entries[i].key, 1);
            set->entries[slot].key = old_entries[i].key;
            set->entries[slot].state = SET_OCCUPIED;
            set->len++;
        }
    }
    free(old_entries);
}

PyFloatSet* float_set_new(void) {
    PyFloatSet* set = (PyFloatSet*)malloc(sizeof(PyFloatSet));
    if (set == NULL) return NULL;
    set->capacity = INITIAL_CAPACITY;
    set->len = 0;
    set->entries = (FloatSetEntry*)calloc(set->capacity, sizeof(FloatSetEntry));
    if (set->entries == NULL) {
        free(set);
        return NULL;
    }
    return set;
}

void float_set_add(PyFloatSet* set, double key) {
    if (set == NULL) return;
    if ((double)(set->len + 1) > set->capacity * LOAD_FACTOR) {
        float_set_grow(set);
    }
    int64_t slot = float_set_find_slot(set, key, 1);
    if (slot >= 0 && set->entries[slot].state != SET_OCCUPIED) {
        set->entries[slot].key = key;
        set->entries[slot].state = SET_OCCUPIED;
        set->len++;
    }
}

int64_t float_set_contains(PyFloatSet* set, double key) {
    if (set == NULL) return 0;
    int64_t slot = float_set_find_slot(set, key, 0);
    return slot >= 0 ? 1 : 0;
}

void float_set_remove(PyFloatSet* set, double key) {
    if (set == NULL) return;
    int64_t slot = float_set_find_slot(set, key, 0);
    if (slot >= 0) {
        set->entries[slot].state = SET_DELETED;
        set->len--;
    }
}

void float_set_discard(PyFloatSet* set, double key) {
    float_set_remove(set, key);
}

void float_set_clear(PyFloatSet* set) {
    if (set == NULL) return;
    for (int64_t i = 0; i < set->capacity; i++) {
        set->entries[i].state = SET_EMPTY;
    }
    set->len = 0;
}

double float_set_pop(PyFloatSet* set) {
    if (set == NULL || set->len == 0) return 0.0;
    for (int64_t i = 0; i < set->capacity; i++) {
        if (set->entries[i].state == SET_OCCUPIED) {
            double result = set->entries[i].key;
            set->entries[i].state = SET_DELETED;
            set->len--;
            return result;
        }
    }
    return 0.0;
}

PyFloatSet* float_set_copy(PyFloatSet* set) {
    if (set == NULL) return float_set_new();
    PyFloatSet* copy = (PyFloatSet*)malloc(sizeof(PyFloatSet));
    if (copy == NULL) return NULL;
    copy->capacity = set->capacity;
    copy->len = set->len;
    copy->entries = (FloatSetEntry*)malloc(copy->capacity * sizeof(FloatSetEntry));
    if (copy->entries == NULL) {
        free(copy);
        return NULL;
    }
    memcpy(copy->entries, set->entries, set->capacity * sizeof(FloatSetEntry));
    return copy;
}

int64_t float_set_len(PyFloatSet* set) {
    return set ? set->len : 0;
}

void print_float_set(PyFloatSet* set) {
    if (set != NULL && set->len == 0) {
        printf("set()");
        return;
    }
    printf("{");
    int first = 1;
    if (set != NULL) {
        for (int64_t i = 0; i < set->capacity; i++) {
            if (set->entries[i].state == SET_OCCUPIED) {
                if (!first) printf(", ");
                printf("%g", set->entries[i].key);
                first = 0;
            }
        }
    }
    printf("}");
}

// ============================================================================
// Bool Set (PyBoolSet) - really just 0/1/both possible
// ============================================================================

typedef struct {
    int8_t key;
    uint8_t state;
} BoolSetEntry;

typedef struct {
    int64_t len;
    int64_t capacity;
    BoolSetEntry* entries;
} PyBoolSet;

static int64_t bool_set_find_slot(PyBoolSet* set, int8_t key, int for_insert) {
    int64_t mask = set->capacity - 1;
    int64_t index = (key ? 1 : 0) & mask;
    int64_t first_deleted = -1;

    for (int64_t i = 0; i < set->capacity; i++) {
        BoolSetEntry* entry = &set->entries[index];
        if (entry->state == SET_EMPTY) {
            return for_insert ? (first_deleted >= 0 ? first_deleted : index) : -1;
        }
        if (entry->state == SET_DELETED) {
            if (first_deleted < 0) first_deleted = index;
        } else if (entry->key == (key ? 1 : 0)) {
            return index;
        }
        index = (index + 1) & mask;
    }
    return for_insert ? first_deleted : -1;
}

PyBoolSet* bool_set_new(void) {
    PyBoolSet* set = (PyBoolSet*)malloc(sizeof(PyBoolSet));
    if (set == NULL) return NULL;
    set->capacity = INITIAL_CAPACITY;
    set->len = 0;
    set->entries = (BoolSetEntry*)calloc(set->capacity, sizeof(BoolSetEntry));
    if (set->entries == NULL) {
        free(set);
        return NULL;
    }
    return set;
}

void bool_set_add(PyBoolSet* set, int8_t key) {
    if (set == NULL) return;
    int64_t slot = bool_set_find_slot(set, key, 1);
    if (slot >= 0 && set->entries[slot].state != SET_OCCUPIED) {
        set->entries[slot].key = key ? 1 : 0;
        set->entries[slot].state = SET_OCCUPIED;
        set->len++;
    }
}

int64_t bool_set_contains(PyBoolSet* set, int8_t key) {
    if (set == NULL) return 0;
    int64_t slot = bool_set_find_slot(set, key, 0);
    return slot >= 0 ? 1 : 0;
}

void bool_set_remove(PyBoolSet* set, int8_t key) {
    if (set == NULL) return;
    int64_t slot = bool_set_find_slot(set, key, 0);
    if (slot >= 0) {
        set->entries[slot].state = SET_DELETED;
        set->len--;
    }
}

void bool_set_discard(PyBoolSet* set, int8_t key) {
    bool_set_remove(set, key);
}

void bool_set_clear(PyBoolSet* set) {
    if (set == NULL) return;
    for (int64_t i = 0; i < set->capacity; i++) {
        set->entries[i].state = SET_EMPTY;
    }
    set->len = 0;
}

int8_t bool_set_pop(PyBoolSet* set) {
    if (set == NULL || set->len == 0) return 0;
    for (int64_t i = 0; i < set->capacity; i++) {
        if (set->entries[i].state == SET_OCCUPIED) {
            int8_t result = set->entries[i].key;
            set->entries[i].state = SET_DELETED;
            set->len--;
            return result;
        }
    }
    return 0;
}

PyBoolSet* bool_set_copy(PyBoolSet* set) {
    if (set == NULL) return bool_set_new();
    PyBoolSet* copy = (PyBoolSet*)malloc(sizeof(PyBoolSet));
    if (copy == NULL) return NULL;
    copy->capacity = set->capacity;
    copy->len = set->len;
    copy->entries = (BoolSetEntry*)malloc(copy->capacity * sizeof(BoolSetEntry));
    if (copy->entries == NULL) {
        free(copy);
        return NULL;
    }
    memcpy(copy->entries, set->entries, set->capacity * sizeof(BoolSetEntry));
    return copy;
}

int64_t bool_set_len(PyBoolSet* set) {
    return set ? set->len : 0;
}

void print_bool_set(PyBoolSet* set) {
    if (set != NULL && set->len == 0) {
        printf("set()");
        return;
    }
    printf("{");
    int first = 1;
    if (set != NULL) {
        for (int64_t i = 0; i < set->capacity; i++) {
            if (set->entries[i].state == SET_OCCUPIED) {
                if (!first) printf(", ");
                printf("%s", set->entries[i].key ? "True" : "False");
                first = 0;
            }
        }
    }
    printf("}");
}

// ============================================================================
// Builtin Functions: sum, sorted
// ============================================================================

// Sum all elements in the set
int64_t set_sum(PySet* set, int64_t start) {
    int64_t sum = start;
    if (set != NULL) {
        for (int64_t i = 0; i < set->capacity; i++) {
            if (set->entries[i].state == SET_OCCUPIED) {
                sum += set->entries[i].key;
            }
        }
    }
    return sum;
}

// Forward declaration for list operations
typedef struct PyList {
    int64_t len;
    int64_t capacity;
    int64_t* data;
} PyList;

extern PyList* list_new(void);
extern PyList* list_with_capacity(int64_t capacity);

// Helper for qsort
static int compare_int64_set(const void* a, const void* b) {
    int64_t va = *(const int64_t*)a;
    int64_t vb = *(const int64_t*)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

// Return a sorted list from set elements
PyList* set_sorted(PySet* set) {
    if (set == NULL || set->len == 0) return list_new();

    PyList* result = list_with_capacity(set->len);
    if (result == NULL) return NULL;

    int64_t j = 0;
    for (int64_t i = 0; i < set->capacity && j < set->len; i++) {
        if (set->entries[i].state == SET_OCCUPIED) {
            result->data[j++] = set->entries[i].key;
        }
    }
    result->len = j;

    // Sort
    qsort(result->data, result->len, sizeof(int64_t), compare_int64_set);

    return result;
}

// ============================================================================
// Set Construction from Iterables
// ============================================================================

// Forward declarations for list
typedef struct {
    int64_t len;
    int64_t capacity;
    int64_t* data;
} PyListFwd;

// Forward declarations for dict
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

// Create set from list (of ints)
PySet* set_from_list(PyListFwd* list) {
    if (list == NULL) return set_new();

    PySet* result = set_new();
    if (result == NULL) return NULL;

    for (int64_t i = 0; i < list->len; i++) {
        set_add(result, list->data[i]);
    }

    return result;
}

// Create set from string (each character becomes its ordinal value)
PySet* set_from_str(const char* s) {
    if (s == NULL) return set_new();

    PySet* result = set_new();
    if (result == NULL) return NULL;

    // Use strlen directly (inline to avoid cross-module linking issues)
    size_t len = strlen(s);
    for (size_t i = 0; i < len; i++) {
        // Add character ordinal value
        set_add(result, (int64_t)(unsigned char)s[i]);
    }

    return result;
}

// Create set from bytes (each byte becomes its value)
PySet* set_from_bytes(const char* s) {
    if (s == NULL) return set_new();

    PySet* result = set_new();
    if (result == NULL) return NULL;

    // Use strlen directly (inline to avoid cross-module linking issues)
    size_t len = strlen(s);
    for (size_t i = 0; i < len; i++) {
        set_add(result, (int64_t)(unsigned char)s[i]);
    }

    return result;
}

// Create set from dict (uses int keys)
PySet* set_from_dict(PyDictFwd* dict) {
    if (dict == NULL) return set_new();

    PySet* result = set_new();
    if (result == NULL) return NULL;

    // Iterate over all occupied slots in dict
    for (int64_t i = 0; i < dict->capacity; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED_FWD) {
            set_add(result, dict->entries[i].key);
        }
    }

    return result;
}

// ============================================================================
// Set Iteration Support
// ============================================================================

typedef struct {
    PySet* set;
    int64_t slot_index;  // Current slot being checked
} SetIterator;

// Create a new iterator for the set
SetIterator* set_iter(PySet* set) {
    SetIterator* iter = (SetIterator*)malloc(sizeof(SetIterator));
    if (iter == NULL) return NULL;
    iter->set = set;
    iter->slot_index = 0;
    return iter;
}

// Get next element (returns 1 if has more, 0 if done)
// Output value is stored in *out
int64_t set_iter_next(SetIterator* iter, int64_t* out) {
    if (iter == NULL || iter->set == NULL) return 0;

    // Find next occupied slot
    while (iter->slot_index < iter->set->capacity) {
        if (iter->set->entries[iter->slot_index].state == SET_OCCUPIED) {
            *out = iter->set->entries[iter->slot_index].key;
            iter->slot_index++;
            return 1;
        }
        iter->slot_index++;
    }
    return 0;
}

// Free the iterator
void set_iter_free(SetIterator* iter) {
    free(iter);
}

// ============================================================================
// any() and all() builtins for sets
// ============================================================================

// any(set) - returns true if any element is non-zero
int64_t set_any(PySet* set) {
    if (set == NULL || set->len == 0) return 0;
    for (int64_t i = 0; i < set->capacity; i++) {
        if (set->entries[i].state == SET_OCCUPIED) {
            if (set->entries[i].key != 0) return 1;
        }
    }
    return 0;
}

// all(set) - returns true if all elements are non-zero
int64_t set_all(PySet* set) {
    if (set == NULL) return 1;
    if (set->len == 0) return 1;
    for (int64_t i = 0; i < set->capacity; i++) {
        if (set->entries[i].state == SET_OCCUPIED) {
            if (set->entries[i].key == 0) return 0;
        }
    }
    return 1;
}

// any(str_set) - returns true if any string is non-empty
int64_t str_set_any(PyStrSet* set) {
    if (set == NULL || set->len == 0) return 0;
    for (int64_t i = 0; i < set->capacity; i++) {
        if (set->entries[i].state == SET_OCCUPIED) {
            if (set->entries[i].key != NULL && set->entries[i].key[0] != '\0') return 1;
        }
    }
    return 0;
}

// all(str_set) - returns true if all strings are non-empty
int64_t str_set_all(PyStrSet* set) {
    if (set == NULL) return 1;
    if (set->len == 0) return 1;
    for (int64_t i = 0; i < set->capacity; i++) {
        if (set->entries[i].state == SET_OCCUPIED) {
            if (set->entries[i].key == NULL || set->entries[i].key[0] == '\0') return 0;
        }
    }
    return 1;
}

// repr(set) - returns string like "{1, 2, 3}"
char* repr_set(PySet* set) {
    if (set == NULL || set->len == 0) return strdup("set()");

    // Estimate size
    size_t est_size = set->len * 22 + 3;
    char* result = (char*)malloc(est_size);
    if (result == NULL) return NULL;

    strcpy(result, "{");
    char buf[32];
    int first = 1;
    for (int64_t i = 0; i < set->capacity; i++) {
        if (set->entries[i].state == SET_OCCUPIED) {
            if (!first) strcat(result, ", ");
            first = 0;
            snprintf(buf, sizeof(buf), "%lld", (long long)set->entries[i].key);
            strcat(result, buf);
        }
    }
    strcat(result, "}");
    return result;
}

// ============================================================================
// frozenset() constructor
// Frozenset is just a set that's treated as immutable
// ============================================================================

// frozenset from string (unique character ordinals)
PySet* frozenset_from_str(const char* s) {
    if (s == NULL) return set_new();
    PySet* result = set_new();
    size_t len = strlen(s);
    for (size_t i = 0; i < len; i++) {
        set_add(result, (int64_t)(unsigned char)s[i]);
    }
    return result;
}

// frozenset from bytes (unique byte values)
PySet* frozenset_from_bytes(const char* s) {
    return frozenset_from_str(s);  // Same as string
}

// frozenset from list (unique elements)
PySet* frozenset_from_list(void* list_ptr) {
    // Cast to our PyList-like structure
    typedef struct { int64_t len; int64_t capacity; int64_t* data; } PyListFS;
    PyListFS* list = (PyListFS*)list_ptr;
    if (list == NULL) return set_new();
    PySet* result = set_new();
    for (int64_t i = 0; i < list->len; i++) {
        set_add(result, list->data[i]);
    }
    return result;
}

// frozenset from set (copy)
PySet* frozenset_from_set(PySet* set) {
    if (set == NULL) return set_new();
    PySet* result = set_new();
    for (int64_t i = 0; i < set->capacity; i++) {
        if (set->entries[i].state == SET_OCCUPIED) {
            set_add(result, set->entries[i].key);
        }
    }
    return result;
}

// frozenset from dict (unique keys)
PySet* frozenset_from_dict(void* dict_ptr) {
    // Dict keys as set elements - simplified for int keys
    typedef struct { int64_t key; int64_t value; uint8_t state; } DictEntryFS;
    typedef struct { int64_t len; int64_t capacity; DictEntryFS* entries; } PyDictFS;
    PyDictFS* dict = (PyDictFS*)dict_ptr;
    if (dict == NULL) return set_new();
    PySet* result = set_new();
    for (int64_t i = 0; i < dict->capacity; i++) {
        if (dict->entries[i].state == 1) {  // DICT_OCCUPIED
            set_add(result, dict->entries[i].key);
        }
    }
    return result;
}
