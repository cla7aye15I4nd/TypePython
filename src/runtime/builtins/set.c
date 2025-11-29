// TypePython Runtime Library - Set Module
// Hash set implementation for set type
//
// Uses open addressing with linear probing (similar to dict but keys only)

#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

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

// Hash function for int keys
static uint64_t hash_int(int64_t key) {
    uint64_t h = (uint64_t)key;
    h ^= h >> 33;
    h *= 0xff51afd7ed558ccdULL;
    h ^= h >> 33;
    h *= 0xc4ceb9fe1a85ec53ULL;
    h ^= h >> 33;
    return h;
}

static int64_t find_slot(PySet* set, int64_t key, int for_insert) {
    uint64_t hash = hash_int(key);
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

// ============================================================================
// Print Support
// ============================================================================

void print_set(PySet* set) {
    printf("{");
    if (set != NULL) {
        int first = 1;
        for (int64_t i = 0; i < set->capacity; i++) {
            if (set->entries[i].state == SET_OCCUPIED) {
                if (!first) printf(", ");
                printf("%ld", set->entries[i].key);
                first = 0;
            }
        }
    }
    printf("}\n");
}
