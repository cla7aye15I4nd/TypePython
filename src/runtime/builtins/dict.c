// TypePython Runtime Library - Dict Module
// Hash table implementation for dict type
//
// Uses open addressing with linear probing

#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#include "../common.h"

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

static int64_t find_slot(PyDict* dict, int64_t key, int for_insert) {
    uint64_t hash = tpy_hash_int(key);
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

// Check if float key exists (treats keys as doubles)
int64_t dict_contains_float(PyDict* dict, double key) {
    if (dict == NULL) return 0;
    // Type-pun the double as int64_t for hashing
    union { double d; int64_t i; } u;
    u.d = key;
    return find_slot(dict, u.i, 0) >= 0 ? 1 : 0;
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

// Find the maximum key in the dict
int64_t dict_max(PyDict* dict) {
    if (dict == NULL || dict->len == 0) return 0;
    int64_t max_key = 0;
    int first = 1;
    for (int64_t i = 0; i < dict->capacity; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED) {
            if (first || dict->entries[i].key > max_key) {
                max_key = dict->entries[i].key;
                first = 0;
            }
        }
    }
    return max_key;
}

// Find the minimum key in the dict
int64_t dict_min(PyDict* dict) {
    if (dict == NULL || dict->len == 0) return 0;
    int64_t min_key = 0;
    int first = 1;
    for (int64_t i = 0; i < dict->capacity; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED) {
            if (first || dict->entries[i].key < min_key) {
                min_key = dict->entries[i].key;
                first = 0;
            }
        }
    }
    return min_key;
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

// ============================================================================
// String-Keyed Dict (PyStrDict)
// ============================================================================

typedef struct {
    char* key;
    int64_t value;
    uint8_t state;
} StrDictEntry;

typedef struct {
    int64_t len;
    int64_t capacity;
    StrDictEntry* entries;
} PyStrDict;

// Hash function for string keys (djb2)
static uint64_t hash_str(const char* key) {
    uint64_t hash = 5381;
    int c;
    while ((c = *key++)) {
        hash = ((hash << 5) + hash) + c;
    }
    return hash;
}

static int64_t str_dict_find_slot(PyStrDict* dict, const char* key, int for_insert) {
    uint64_t hash = hash_str(key);
    int64_t mask = dict->capacity - 1;
    int64_t index = hash & mask;
    int64_t first_deleted = -1;

    for (int64_t i = 0; i < dict->capacity; i++) {
        StrDictEntry* entry = &dict->entries[index];
        if (entry->state == DICT_EMPTY) {
            return for_insert ? (first_deleted >= 0 ? first_deleted : index) : -1;
        }
        if (entry->state == DICT_DELETED) {
            if (first_deleted < 0) first_deleted = index;
        } else if (strcmp(entry->key, key) == 0) {
            return index;
        }
        index = (index + 1) & mask;
    }
    return for_insert ? first_deleted : -1;
}

static PyStrDict* str_dict_resize(PyStrDict* dict, int64_t new_capacity) {
    StrDictEntry* old_entries = dict->entries;
    int64_t old_capacity = dict->capacity;

    dict->entries = (StrDictEntry*)calloc(new_capacity, sizeof(StrDictEntry));
    if (dict->entries == NULL) {
        dict->entries = old_entries;
        return NULL;
    }
    dict->capacity = new_capacity;
    dict->len = 0;

    for (int64_t i = 0; i < old_capacity; i++) {
        if (old_entries[i].state == DICT_OCCUPIED) {
            int64_t slot = str_dict_find_slot(dict, old_entries[i].key, 1);
            dict->entries[slot].key = old_entries[i].key;
            dict->entries[slot].value = old_entries[i].value;
            dict->entries[slot].state = DICT_OCCUPIED;
            dict->len++;
        }
    }

    free(old_entries);
    return dict;
}

PyStrDict* str_dict_new(void) {
    PyStrDict* dict = (PyStrDict*)malloc(sizeof(PyStrDict));
    if (dict == NULL) return NULL;
    dict->len = 0;
    dict->capacity = INITIAL_CAPACITY;
    dict->entries = (StrDictEntry*)calloc(INITIAL_CAPACITY, sizeof(StrDictEntry));
    if (dict->entries == NULL) {
        free(dict);
        return NULL;
    }
    return dict;
}

void str_dict_setitem(PyStrDict* dict, const char* key, int64_t value) {
    if (dict == NULL) return;

    if ((double)dict->len / dict->capacity >= LOAD_FACTOR) {
        str_dict_resize(dict, dict->capacity * 2);
    }

    int64_t slot = str_dict_find_slot(dict, key, 1);
    if (slot < 0) return;

    if (dict->entries[slot].state != DICT_OCCUPIED) {
        dict->entries[slot].key = strdup(key);
        dict->len++;
    }
    dict->entries[slot].value = value;
    dict->entries[slot].state = DICT_OCCUPIED;
}

int64_t str_dict_getitem(PyStrDict* dict, const char* key) {
    if (dict == NULL) return 0;
    int64_t slot = str_dict_find_slot(dict, key, 0);
    if (slot < 0) return 0;
    return dict->entries[slot].value;
}

int64_t str_dict_contains(PyStrDict* dict, const char* key) {
    if (dict == NULL) return 0;
    return str_dict_find_slot(dict, key, 0) >= 0 ? 1 : 0;
}

int64_t str_dict_len(PyStrDict* dict) {
    if (dict == NULL) return 0;
    return dict->len;
}

// Find min key (lexicographic)
char* str_dict_min(PyStrDict* dict) {
    if (dict == NULL || dict->len == 0) return strdup("");
    char* min_key = NULL;
    for (int64_t i = 0; i < dict->capacity; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED) {
            if (min_key == NULL || strcmp(dict->entries[i].key, min_key) < 0) {
                min_key = dict->entries[i].key;
            }
        }
    }
    return min_key ? strdup(min_key) : strdup("");
}

// Find max key (lexicographic)
char* str_dict_max(PyStrDict* dict) {
    if (dict == NULL || dict->len == 0) return strdup("");
    char* max_key = NULL;
    for (int64_t i = 0; i < dict->capacity; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED) {
            if (max_key == NULL || strcmp(dict->entries[i].key, max_key) > 0) {
                max_key = dict->entries[i].key;
            }
        }
    }
    return max_key ? strdup(max_key) : strdup("");
}

// Copy a string dict
PyStrDict* str_dict_copy(PyStrDict* dict) {
    if (dict == NULL) return str_dict_new();
    PyStrDict* copy = (PyStrDict*)malloc(sizeof(PyStrDict));
    if (copy == NULL) return NULL;
    copy->len = 0;
    copy->capacity = dict->capacity;
    copy->entries = (StrDictEntry*)calloc(copy->capacity, sizeof(StrDictEntry));
    if (copy->entries == NULL) {
        free(copy);
        return NULL;
    }
    // Copy all entries
    for (int64_t i = 0; i < dict->capacity; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED) {
            copy->entries[i].key = strdup(dict->entries[i].key);
            copy->entries[i].value = dict->entries[i].value;
            copy->entries[i].state = DICT_OCCUPIED;
            copy->len++;
        }
    }
    return copy;
}

// Merge two string dicts: dict1 | dict2
PyStrDict* str_dict_merge(PyStrDict* dict1, PyStrDict* dict2) {
    PyStrDict* result = str_dict_copy(dict1);
    if (result == NULL) return NULL;
    // Update with dict2 entries
    if (dict2 != NULL) {
        for (int64_t i = 0; i < dict2->capacity; i++) {
            if (dict2->entries[i].state == DICT_OCCUPIED) {
                str_dict_setitem(result, dict2->entries[i].key, dict2->entries[i].value);
            }
        }
    }
    return result;
}

// Get a value with default: dict.get(key, default)
int64_t str_dict_get(PyStrDict* dict, const char* key, int64_t default_value) {
    if (dict == NULL) return default_value;
    int64_t slot = str_dict_find_slot(dict, key, 0);
    if (slot < 0) return default_value;
    return dict->entries[slot].value;
}

// Pop a value: dict.pop(key) - removes and returns the value
int64_t str_dict_pop(PyStrDict* dict, const char* key) {
    if (dict == NULL) return 0;
    int64_t slot = str_dict_find_slot(dict, key, 0);
    if (slot < 0) return 0;
    int64_t value = dict->entries[slot].value;
    free(dict->entries[slot].key);
    dict->entries[slot].key = NULL;
    dict->entries[slot].state = DICT_DELETED;
    dict->len--;
    return value;
}

// Clear all entries: dict.clear()
void str_dict_clear(PyStrDict* dict) {
    if (dict == NULL) return;
    for (int64_t i = 0; i < dict->capacity; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED) {
            free(dict->entries[i].key);
            dict->entries[i].key = NULL;
            dict->entries[i].state = DICT_EMPTY;
        }
    }
    dict->len = 0;
}

// Update with another dict: dict.update(other)
void str_dict_update(PyStrDict* dict, PyStrDict* other) {
    if (dict == NULL || other == NULL) return;
    for (int64_t i = 0; i < other->capacity; i++) {
        if (other->entries[i].state == DICT_OCCUPIED) {
            str_dict_setitem(dict, other->entries[i].key, other->entries[i].value);
        }
    }
}

// Set default: dict.setdefault(key, default) - insert if not present
int64_t str_dict_setdefault(PyStrDict* dict, const char* key, int64_t default_value) {
    if (dict == NULL) return default_value;
    int64_t slot = str_dict_find_slot(dict, key, 0);
    if (slot >= 0) {
        return dict->entries[slot].value;
    }
    // Key not found, insert default
    str_dict_setitem(dict, key, default_value);
    return default_value;
}

void print_str_dict(PyStrDict* dict) {
    printf("{");
    if (dict != NULL) {
        int first = 1;
        for (int64_t i = 0; i < dict->capacity; i++) {
            if (dict->entries[i].state == DICT_OCCUPIED) {
                if (!first) printf(", ");
                printf("'%s': %ld", dict->entries[i].key, dict->entries[i].value);
                first = 0;
            }
        }
    }
    printf("}");
}

// ============================================================================
// Sorted/Reversed Functions
// ============================================================================

// Forward declaration for list
typedef struct {
    int64_t len;
    int64_t capacity;
    int64_t* data;
} PyListDict;

// External list function
extern PyListDict* list_with_capacity(int64_t capacity);

// Helper for int comparison in qsort
static int cmp_int64_dict(const void* a, const void* b) {
    int64_t va = *(const int64_t*)a;
    int64_t vb = *(const int64_t*)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

// Return sorted list of dict keys (int-keyed dict)
PyListDict* dict_sorted(PyDict* dict) {
    if (dict == NULL || dict->len == 0) {
        PyListDict* result = list_with_capacity(0);
        if (result) result->len = 0;
        return result;
    }

    PyListDict* result = list_with_capacity(dict->len);
    if (result == NULL) return NULL;

    // Collect keys
    int64_t j = 0;
    for (int64_t i = 0; i < dict->capacity && j < dict->len; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED) {
            result->data[j++] = dict->entries[i].key;
        }
    }
    result->len = j;

    // Sort
    qsort(result->data, j, sizeof(int64_t), cmp_int64_dict);

    return result;
}

// Return reversed list of dict keys (reversed insertion order - approximated by reversed sorted)
PyListDict* dict_reversed(PyDict* dict) {
    if (dict == NULL || dict->len == 0) {
        PyListDict* result = list_with_capacity(0);
        if (result) result->len = 0;
        return result;
    }

    PyListDict* result = list_with_capacity(dict->len);
    if (result == NULL) return NULL;

    // Collect keys
    int64_t j = 0;
    for (int64_t i = 0; i < dict->capacity && j < dict->len; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED) {
            result->data[j++] = dict->entries[i].key;
        }
    }
    result->len = j;

    // Reverse in place
    for (int64_t i = 0; i < j / 2; i++) {
        int64_t tmp = result->data[i];
        result->data[i] = result->data[j - 1 - i];
        result->data[j - 1 - i] = tmp;
    }

    return result;
}

// ============================================================================
// Dict Iteration Support
// ============================================================================

typedef struct {
    PyDict* dict;
    int64_t slot_index;  // Current slot being checked
} DictIterator;

// Create a new iterator for the dict (iterates over keys)
DictIterator* dict_iter(PyDict* dict) {
    DictIterator* iter = (DictIterator*)malloc(sizeof(DictIterator));
    if (iter == NULL) return NULL;
    iter->dict = dict;
    iter->slot_index = 0;
    return iter;
}

// Get next key (returns 1 if has more, 0 if done)
// Output key is stored in *out
int64_t dict_iter_next(DictIterator* iter, int64_t* out) {
    if (iter == NULL || iter->dict == NULL) return 0;

    // Find next occupied slot
    while (iter->slot_index < iter->dict->capacity) {
        if (iter->dict->entries[iter->slot_index].state == DICT_OCCUPIED) {
            *out = iter->dict->entries[iter->slot_index].key;
            iter->slot_index++;
            return 1;
        }
        iter->slot_index++;
    }
    return 0;
}

// Free the iterator
void dict_iter_free(DictIterator* iter) {
    free(iter);
}

// ============================================================================
// Dict keys(), values(), items() View Iterators
// ============================================================================

// Keys iterator (same as dict_iter, but named differently for method call)
DictIterator* dict_keys(PyDict* dict) {
    return dict_iter(dict);
}

// Values iterator
typedef struct {
    PyDict* dict;
    int64_t slot_index;
} DictValuesIterator;

DictValuesIterator* dict_values(PyDict* dict) {
    DictValuesIterator* iter = (DictValuesIterator*)malloc(sizeof(DictValuesIterator));
    if (iter == NULL) return NULL;
    iter->dict = dict;
    iter->slot_index = 0;
    return iter;
}

// Get next value (returns 1 if has more, 0 if done)
int64_t dict_values_next(DictValuesIterator* iter, int64_t* out) {
    if (iter == NULL || iter->dict == NULL) return 0;

    while (iter->slot_index < iter->dict->capacity) {
        if (iter->dict->entries[iter->slot_index].state == DICT_OCCUPIED) {
            *out = iter->dict->entries[iter->slot_index].value;
            iter->slot_index++;
            return 1;
        }
        iter->slot_index++;
    }
    return 0;
}

void dict_values_free(DictValuesIterator* iter) {
    free(iter);
}

// Items iterator - returns key/value pairs
typedef struct {
    PyDict* dict;
    int64_t slot_index;
} DictItemsIterator;

DictItemsIterator* dict_items(PyDict* dict) {
    DictItemsIterator* iter = (DictItemsIterator*)malloc(sizeof(DictItemsIterator));
    if (iter == NULL) return NULL;
    iter->dict = dict;
    iter->slot_index = 0;
    return iter;
}

// Get next (key, value) pair. Returns 1 if has more, 0 if done.
// key is returned via *key_out, value via *value_out
int64_t dict_items_next(DictItemsIterator* iter, int64_t* key_out, int64_t* value_out) {
    if (iter == NULL || iter->dict == NULL) return 0;

    while (iter->slot_index < iter->dict->capacity) {
        if (iter->dict->entries[iter->slot_index].state == DICT_OCCUPIED) {
            *key_out = iter->dict->entries[iter->slot_index].key;
            *value_out = iter->dict->entries[iter->slot_index].value;
            iter->slot_index++;
            return 1;
        }
        iter->slot_index++;
    }
    return 0;
}

void dict_items_free(DictItemsIterator* iter) {
    free(iter);
}

// ============================================================================
// String Dict keys(), values(), items() View Iterators
// ============================================================================

// String dict keys iterator
typedef struct {
    PyStrDict* dict;
    int64_t slot_index;
} StrDictIterator;

StrDictIterator* str_dict_keys(PyStrDict* dict) {
    StrDictIterator* iter = (StrDictIterator*)malloc(sizeof(StrDictIterator));
    if (iter == NULL) return NULL;
    iter->dict = dict;
    iter->slot_index = 0;
    return iter;
}

// Also use for str_dict_iter
StrDictIterator* str_dict_iter(PyStrDict* dict) {
    return str_dict_keys(dict);
}

// Get next key (returns char*, or NULL if done)
char* str_dict_keys_next(StrDictIterator* iter, int8_t* exhausted) {
    if (iter == NULL || iter->dict == NULL) {
        *exhausted = 1;
        return NULL;
    }

    while (iter->slot_index < iter->dict->capacity) {
        if (iter->dict->entries[iter->slot_index].state == DICT_OCCUPIED) {
            char* key = iter->dict->entries[iter->slot_index].key;
            iter->slot_index++;
            *exhausted = 0;
            return key;
        }
        iter->slot_index++;
    }
    *exhausted = 1;
    return NULL;
}

void str_dict_keys_free(StrDictIterator* iter) {
    free(iter);
}

// String dict values iterator
typedef struct {
    PyStrDict* dict;
    int64_t slot_index;
} StrDictValuesIterator;

StrDictValuesIterator* str_dict_values(PyStrDict* dict) {
    StrDictValuesIterator* iter = (StrDictValuesIterator*)malloc(sizeof(StrDictValuesIterator));
    if (iter == NULL) return NULL;
    iter->dict = dict;
    iter->slot_index = 0;
    return iter;
}

int64_t str_dict_values_next(StrDictValuesIterator* iter, int64_t* out) {
    if (iter == NULL || iter->dict == NULL) return 0;

    while (iter->slot_index < iter->dict->capacity) {
        if (iter->dict->entries[iter->slot_index].state == DICT_OCCUPIED) {
            *out = iter->dict->entries[iter->slot_index].value;
            iter->slot_index++;
            return 1;
        }
        iter->slot_index++;
    }
    return 0;
}

void str_dict_values_free(StrDictValuesIterator* iter) {
    free(iter);
}

// String dict items iterator
typedef struct {
    PyStrDict* dict;
    int64_t slot_index;
} StrDictItemsIterator;

StrDictItemsIterator* str_dict_items(PyStrDict* dict) {
    StrDictItemsIterator* iter = (StrDictItemsIterator*)malloc(sizeof(StrDictItemsIterator));
    if (iter == NULL) return NULL;
    iter->dict = dict;
    iter->slot_index = 0;
    return iter;
}

// Get next (key, value) pair. Returns 1 if has more, 0 if done.
// key is returned via *key_out (char*), value via *value_out (int64_t)
int64_t str_dict_items_next(StrDictItemsIterator* iter, char** key_out, int64_t* value_out) {
    if (iter == NULL || iter->dict == NULL) return 0;

    while (iter->slot_index < iter->dict->capacity) {
        if (iter->dict->entries[iter->slot_index].state == DICT_OCCUPIED) {
            *key_out = iter->dict->entries[iter->slot_index].key;
            *value_out = iter->dict->entries[iter->slot_index].value;
            iter->slot_index++;
            return 1;
        }
        iter->slot_index++;
    }
    return 0;
}

void str_dict_items_free(StrDictItemsIterator* iter) {
    free(iter);
}

// ============================================================================
// any() and all() builtins for dicts
// ============================================================================

// Forward declaration for int dict structure
typedef struct {
    int64_t key;
    int64_t value;
    uint8_t state;
} IntDictEntry;

typedef struct {
    int64_t len;
    int64_t capacity;
    IntDictEntry* entries;
} PyIntDict;

// any(dict) - returns true if any key is non-zero (for int-keyed dicts)
int64_t dict_any(PyIntDict* dict) {
    if (dict == NULL || dict->len == 0) return 0;
    for (int64_t i = 0; i < dict->capacity; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED) {
            if (dict->entries[i].key != 0) return 1;
        }
    }
    return 0;
}

// all(dict) - returns true if all keys are non-zero (for int-keyed dicts)
int64_t dict_all(PyIntDict* dict) {
    if (dict == NULL) return 1;
    if (dict->len == 0) return 1;
    for (int64_t i = 0; i < dict->capacity; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED) {
            if (dict->entries[i].key == 0) return 0;
        }
    }
    return 1;
}

// any(str_dict) - returns true if any key is non-empty
int64_t str_dict_any(PyStrDict* dict) {
    if (dict == NULL || dict->len == 0) return 0;
    for (int64_t i = 0; i < dict->capacity; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED) {
            if (dict->entries[i].key != NULL && dict->entries[i].key[0] != '\0') return 1;
        }
    }
    return 0;
}

// all(str_dict) - returns true if all keys are non-empty
int64_t str_dict_all(PyStrDict* dict) {
    if (dict == NULL) return 1;
    if (dict->len == 0) return 1;
    for (int64_t i = 0; i < dict->capacity; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED) {
            if (dict->entries[i].key == NULL || dict->entries[i].key[0] == '\0') return 0;
        }
    }
    return 1;
}

// repr(dict) - returns string like "{'a': 1}"
char* repr_str_dict(PyStrDict* dict) {
    if (dict == NULL || dict->len == 0) return strdup("{}");

    // Estimate size
    size_t est_size = dict->len * 50 + 3;
    char* result = (char*)malloc(est_size);
    if (result == NULL) return NULL;

    strcpy(result, "{");
    char buf[64];
    int first = 1;
    for (int64_t i = 0; i < dict->capacity; i++) {
        if (dict->entries[i].state == DICT_OCCUPIED) {
            if (!first) strcat(result, ", ");
            first = 0;
            snprintf(buf, sizeof(buf), "'%s': %lld",
                     dict->entries[i].key ? dict->entries[i].key : "",
                     (long long)dict->entries[i].value);
            strcat(result, buf);
        }
    }
    strcat(result, "}");
    return result;
}
