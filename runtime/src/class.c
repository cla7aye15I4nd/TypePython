#include "runtime.h"
#include <stdlib.h>
#include <string.h>

// Allocate memory for a new class instance
void* class_new(int64_t size) {
    void* instance = malloc((size_t)size);
    if (instance == NULL) {
        rt_panic("Failed to allocate memory for class instance");
    }
    // Zero-initialize all fields
    memset(instance, 0, (size_t)size);
    return instance;
}
