// TypePython Runtime Library - Dict Module
// Hash table implementation for dict type
//
// Uses open addressing with linear probing

#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

// ============================================================================
// Dict Data Structure
// ============================================================================

#define DICT_EMPTY    0
#define DICT_OCCUPIED 1
#define DICT_DELETED  2

typedef struct {
    int64_t key;
    int64_t value;
    uint8_t state;
} DictEntry;

typedef struct {
    int64_t len;       // Number of items
    int64_t capacity;  // Table size
    DictEntry* entries;
} PyDict;

#define INITIAL_CAPACITY 16
#define LOAD_FACTOR 0.75

// ============================================================================
// Internal Helper Functions
// ============================================================================

// Hash function for int keys (simple identity hash for now)
static uint64_t hash_int(int64_t key) {
    // Mix the bits to distribute evenly
    uint64_t h = (uint64_t)key;
    h ^= h >> 33;
    h *= 0xff51afd7ed558ccdULL;
    h ^= h >> 33;
    h *= 0xc4ceb9fe1a85ec53ULL;
    h ^= h >> 33;
    return h;
}

static int64_t find_slot(PyDict* dict, int64_t key, int for_insert) {
    uint64_t hash = hash_int(key);
    int64_t mask = dict->capacity - 1;
    int64_t index = hash & mask;
    int64_t first_deleted = -1;

    for (int64_t i = 0; i < dict->capacity; i++) {
        DictEntry* entry = &dict->entries[index];
        if (entry->state == DICT_EMPTY) {
            return for_insert ? (first_deleted >= 0 ? first_deleted : index) : -1;
        }
        if (entry->state == DICT_DELETED) {
            if (first_deleted < 0) first_deleted = index;
        } else if (entry->key == key) {
            return index;
        }
        index = (index + 1) & mask;
    }
    return for_insert ? first_deleted : -1;
}

static PyDict* dict_resize(PyDict* dict, int64_t new_capacity) {
    DictEntry* old_entries = dict->entries;
    int64_t old_capacity = dict->capacity;

    dict->entries = (DictEntry*)calloc(new_capacity, sizeof(DictEntry));
    if (dict->entries == NULL) {
        dict->entries = old_entries;
        return NULL;
    }
    dict->capacity = new_capacity;
    dict->len = 0;

    // Rehash all entries
    for (int64_t i = 0; i < old_capacity; i++) {
        if (old_entries[i].state == DICT_OCCUPIED) {
            int64_t slot = find_slot(dict, old_entries[i].key, 1);
            dict->entries[slot].key = old_entries[i].key;
            dict->entries[slot].value = old_entries[i].value;
            dict->entries[slot].state = DICT_OCCUPIED;
            dict->len++;
        }
    }

    free(old_entries);
    return dict;
}

// ============================================================================
// Dict Core Functions
// ============================================================================

// Create a new empty dict
PyDict* dict_new(void) {
    PyDict* dict = (PyDict*)malloc(sizeof(PyDict));
    if (dict == NULL) return NULL;
    dict->len = 0;
    dict->capacity = INITIAL_CAPACITY;
    dict->entries = (DictEntry*)calloc(INITIAL_CAPACITY, sizeof(DictEntry));
    if (dict->entries == NULL) {
        free(dict);
        return NULL;
    }
    return dict;
}

// Get the length of the dict
int64_t dict_len(PyDict* dict) {
    if (dict == NULL) return 0;
    return dict->len;
}

// Get item by key (returns 0 if not found - caller should check with contains)
int64_t dict_getitem(PyDict* dict, int64_t key) {
    if (dict == NULL) return 0;
    int64_t slot = find_slot(dict, key, 0);
    if (slot < 0) return 0;
    return dict->entries[slot].value;
}

// Set item by key
void dict_setitem(PyDict* dict, int64_t key, int64_t value) {
    if (dict == NULL) return;

    // Resize if needed
    if ((dict->len + 1) > (int64_t)(dict->capacity * LOAD_FACTOR)) {
        if (dict_resize(dict, dict->capacity * 2) == NULL) return;
    }

    int64_t slot = find_slot(dict, key, 1);
    if (slot < 0) return;

    if (dict->entries[slot].state != DICT_OCCUPIED) {
        dict->len++;
    }
    dict->entries[slot].key = key;
    dict->entries[slot].value = value;
    dict->entries[slot].state = DICT_OCCUPIED;
}

// Delete item by key
void dict_delitem(PyDict* dict, int64_t key) {
    if (dict == NULL) return;
    int64_t slot = find_slot(dict, key, 0);
    if (slot < 0) return;
    dict->entries[slot].state = DICT_DELETED;
    dict->len--;
}

// Check if key exists
int64_t dict_contains(PyDict* dict, int64_t key) {
    if (dict == NULL) return 0;
    return find_slot(dict, key, 0) >= 0 ? 1 : 0;
}

// Get with default value
int64_t dict_get(PyDict* dict, int64_t key, int64_t default_val) {
    if (dict == NULL) return default_val;
    int64_t slot = find_slot(dict, key, 0);
    if (slot < 0) return default_val;
    return dict->entries[slot].value;
}

// Clear all items
void dict_clear(PyDict* dict) {
    if (dict == NULL) return;
    memset(dict->entries, 0, dict->capacity * sizeof(DictEntry));
    dict->len = 0;
}

// Create a shallow copy
PyDict* dict_copy(PyDict* dict) {
    if (dict == NULL) return dict_new();

    PyDict* copy = (PyDict*)malloc(sizeof(PyDict));
    if (copy == NULL) return NULL;

    copy->len = dict->len;
    copy->capacity = dict->capacity;
    copy->entries = (DictEntry*)malloc(dict->capacity * sizeof(DictEntry));
    if (copy->entries == NULL) {
        free(copy);
        return NULL;
    }
    memcpy(copy->entries, dict->entries, dict->capacity * sizeof(DictEntry));
    return copy;
}

// Update with another dict
void dict_update(PyDict* dict, PyDict* other) {
    if (dict == NULL || other == NULL) return;
    for (int64_t i = 0; i < other->capacity; i++) {
        if (other->entries[i].state == DICT_OCCUPIED) {
            dict_setitem(dict, other->entries[i].key, other->entries[i].value);
        }
    }
}

// Pop key, return value
int64_t dict_pop(PyDict* dict, int64_t key) {
    if (dict == NULL) return 0;
    int64_t slot = find_slot(dict, key, 0);
    if (slot < 0) return 0;
    int64_t value = dict->entries[slot].value;
    dict->entries[slot].state = DICT_DELETED;
    dict->len--;
    return value;
}

// Check equality
int64_t dict_eq(PyDict* dict1, PyDict* dict2) {
    int64_t len1 = dict1 ? dict1->len : 0;
    int64_t len2 = dict2 ? dict2->len : 0;

    if (len1 != len2) return 0;
    if (len1 == 0) return 1;

    // Check all keys in dict1 exist in dict2 with same values
    for (int64_t i = 0; i < dict1->capacity; i++) {
        if (dict1->entries[i].state == DICT_OCCUPIED) {
            int64_t slot = find_slot(dict2, dict1->entries[i].key, 0);
            if (slot < 0) return 0;
            if (dict2->entries[slot].value != dict1->entries[i].value) return 0;
        }
    }
    return 1;
}

// Merge two dicts (Python 3.9+ | operator)
PyDict* dict_merge(PyDict* dict1, PyDict* dict2) {
    PyDict* result = dict_copy(dict1);
    if (result == NULL) return NULL;
    dict_update(result, dict2);
    return result;
}

// Pop an arbitrary item (key, value) - returns key, sets *value_out
// Returns 0 if dict is empty (caller should check len first)
int64_t dict_popitem(PyDict* dict, int64_t* value_out) {
    if (dict == NULL || dict->len == 0) {
        if (value_out) *value_out = 0;
        return 0;
    }
    // Find first occupied slot
    for (int64_t i = 0; i < dict->capacity; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED) {
            int64_t key = dict->entries[i].key;
            if (value_out) *value_out = dict->entries[i].value;
            dict->entries[i].state = DICT_DELETED;
            dict->len--;
            return key;
        }
    }
    if (value_out) *value_out = 0;
    return 0;
}

// Get value or set default if key doesn't exist
int64_t dict_setdefault(PyDict* dict, int64_t key, int64_t default_val) {
    if (dict == NULL) return default_val;
    int64_t slot = find_slot(dict, key, 0);
    if (slot >= 0) {
        return dict->entries[slot].value;
    }
    // Key doesn't exist, insert default
    dict_setitem(dict, key, default_val);
    return default_val;
}

// ============================================================================
// Print Support
// ============================================================================

void print_dict(PyDict* dict) {
    printf("{");
    if (dict != NULL) {
        int first = 1;
        for (int64_t i = 0; i < dict->capacity; i++) {
            if (dict->entries[i].state == DICT_OCCUPIED) {
                if (!first) printf(", ");
                printf("%ld: %ld", dict->entries[i].key, dict->entries[i].value);
                first = 0;
            }
        }
    }
    printf("}");
}
