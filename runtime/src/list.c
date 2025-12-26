#include "runtime.h"
#include <stdlib.h>
#include <stdio.h>

List* LIST_METHOD(__init__)(void) {
    List* list = (List*)malloc(sizeof(List));
    if (list == NULL) {
        rt_panic("Failed to allocate memory for list");
    }

    list->cap = 8;
    list->len = 0;
    list->data = (int64_t*)malloc(sizeof(int64_t) * list->cap);

    if (list->data == NULL) {
        rt_panic("Failed to allocate memory for list data");
    }

    return list;
}

void LIST_METHOD(append)(List* list, int64_t value) {
    if (list == NULL) {
        rt_panic("Cannot append to NULL list");
    }

    if (list->len == list->cap) {
        list->cap *= 2;
        int64_t* new_data = (int64_t*)realloc(list->data, sizeof(int64_t) * list->cap);
        if (new_data == NULL) {
            rt_panic("Failed to reallocate memory for list");
        }
        list->data = new_data;
    }

    list->data[list->len++] = value;
}

int64_t LIST_METHOD(__getitem__)(List* list, int64_t index) {
    if (list == NULL) {
        rt_panic("Cannot get from NULL list");
    }
    if (index < 0 || index >= list->len) {
        rt_panic_index("Index out of bounds", index, list->len);
    }
    return list->data[index];
}

void LIST_METHOD(__setitem__)(List* list, int64_t index, int64_t value) {
    if (list == NULL) {
        rt_panic("Cannot set in NULL list");
    }
    if (index < 0 || index >= list->len) {
        rt_panic_index("Index out of bounds", index, list->len);
    }
    list->data[index] = value;
}

int64_t LIST_METHOD(__len__)(List* list) {
    if (list == NULL) {
        rt_panic("Cannot get length of NULL list");
    }
    return list->len;
}

void LIST_METHOD(free)(List* list) {
    if (list != NULL) {
        free(list->data);
        free(list);
    }
}

String* LIST_METHOD(__repr__)(List* list) {
    if (list == NULL || list->len == 0) {
        return STR_METHOD(from_literal)("[]", 2);
    }

    // Max: "[" + 21 chars per int + ", " separators + "]"
    int64_t max_len = 2 + (21 * list->len) + (2 * (list->len - 1));
    String* result = (String*)malloc(sizeof(String) + max_len + 1);
    if (result == NULL) return NULL;

    int64_t pos = 0;
    result->data[pos++] = '[';

    for (int64_t i = 0; i < list->len; i++) {
        if (i > 0) {
            result->data[pos++] = ',';
            result->data[pos++] = ' ';
        }
        pos += snprintf(result->data + pos, 22, "%ld", list->data[i]);
    }

    result->data[pos++] = ']';
    result->data[pos] = '\0';
    result->len = pos;

    return result;
}

String* LIST_METHOD(__str__)(List* list) {
    return LIST_METHOD(__repr__)(list);
}

// ============================================================================
// List Iterator
// ============================================================================

ListIterator* LIST_METHOD(__iter__)(List* list) {
    ListIterator* iter = (ListIterator*)malloc(sizeof(ListIterator));
    if (iter == NULL) {
        rt_panic("Failed to allocate memory for list iterator");
    }
    iter->list = list;
    iter->index = 0;
    return iter;
}

ListIterator* LIST_ITERATOR_METHOD(__iter__)(ListIterator* iter) {
    return iter;
}

int64_t LIST_ITERATOR_METHOD(__next__)(ListIterator* iter) {
    if (iter == NULL) {
        rt_panic("Cannot iterate with NULL iterator");
    }
    if (iter->list == NULL) {
        rt_panic("Cannot iterate over NULL list");
    }

    if (iter->index >= iter->list->len) {
        __pyc_raise(__pyc_stop_iteration());
        return 0;
    }

    return iter->list->data[iter->index++];
}

void LIST_ITERATOR_METHOD(__dealloc__)(ListIterator* iter) {
    free(iter);
}
