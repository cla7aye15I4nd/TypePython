// TypePython Runtime Library - Range Module
// Range iterator implementation for for loops
//
// Memory layout: PyRange struct with start, stop, step
// Range is an immutable sequence of integers

#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>

// ============================================================================
// Range Data Structure
// ============================================================================

typedef struct {
    int64_t start;
    int64_t stop;
    int64_t step;
} PyRange;

// Iterator for range - tracks current position
typedef struct {
    PyRange* range;
    int64_t current;
} PyRangeIter;

// ============================================================================
// Range Core Functions
// ============================================================================

// Create a new range with start, stop, step
PyRange* range_new(int64_t start, int64_t stop, int64_t step) {
    if (step == 0) {
        // ValueError: range() arg 3 must not be zero
        fprintf(stderr, "ValueError: range() arg 3 must not be zero\n");
        return NULL;
    }
    PyRange* range = (PyRange*)malloc(sizeof(PyRange));
    if (range == NULL) return NULL;
    range->start = start;
    range->stop = stop;
    range->step = step;
    return range;
}

// Create range(stop) - equivalent to range(0, stop, 1)
PyRange* range_new1(int64_t stop) {
    return range_new(0, stop, 1);
}

// Create range(start, stop) - equivalent to range(start, stop, 1)
PyRange* range_new2(int64_t start, int64_t stop) {
    return range_new(start, stop, 1);
}

// Get the length of the range
int64_t range_len(PyRange* range) {
    if (range == NULL) return 0;

    int64_t step = range->step;
    int64_t diff = range->stop - range->start;

    // Check if range is empty
    if (step > 0 && diff <= 0) return 0;
    if (step < 0 && diff >= 0) return 0;

    // Calculate length: ceil(|diff| / |step|)
    if (step > 0) {
        return (diff + step - 1) / step;
    } else {
        return (diff + step + 1) / step;
    }
}

// Check if value is in range (for 'in' operator)
int64_t range_contains(PyRange* range, int64_t value) {
    if (range == NULL) return 0;

    int64_t step = range->step;
    int64_t start = range->start;
    int64_t stop = range->stop;

    // Check bounds
    if (step > 0) {
        if (value < start || value >= stop) return 0;
    } else {
        if (value > start || value <= stop) return 0;
    }

    // Check if value is on the step
    return (value - start) % step == 0;
}

// Get item at index (supports negative indexing)
int64_t range_getitem(PyRange* range, int64_t index) {
    if (range == NULL) return 0;

    int64_t len = range_len(range);

    // Normalize negative index
    if (index < 0) {
        index = len + index;
    }

    // Bounds check
    if (index < 0 || index >= len) {
        fprintf(stderr, "IndexError: range object index out of range\n");
        return 0;
    }

    return range->start + index * range->step;
}

// Create a reversed range - returns a new range that iterates in reverse order
// reversed(range(start, stop, step)) is equivalent to range(last_elem, start-step, -step)
// where last_elem is the last element in the original range
PyRange* range_reversed(PyRange* range) {
    if (range == NULL) return NULL;

    int64_t len = range_len(range);
    if (len == 0) {
        // Empty range reversed is still empty
        return range_new(0, 0, 1);
    }

    // Calculate the last element in the original range
    // last = start + (len - 1) * step
    int64_t last = range->start + (len - 1) * range->step;

    // Reversed range: start at last, go to start-step (exclusive), with negated step
    // For range(0, 5, 1) -> reversed gives range(4, -1, -1)
    // For range(0, 10, 2) -> [0,2,4,6,8] -> reversed is range(8, -2, -2) = [8,6,4,2,0]
    int64_t new_start = last;
    int64_t new_stop = range->start - range->step;
    int64_t new_step = -range->step;

    return range_new(new_start, new_stop, new_step);
}

// ============================================================================
// Range Iterator Functions
// ============================================================================

// Create an iterator for the range
PyRangeIter* range_iter(PyRange* range) {
    if (range == NULL) return NULL;

    PyRangeIter* iter = (PyRangeIter*)malloc(sizeof(PyRangeIter));
    if (iter == NULL) return NULL;

    iter->range = range;
    iter->current = range->start;
    return iter;
}

// Get next value from iterator, returns 1 if value available, 0 if exhausted
// The value is stored in *out_value
int64_t range_iter_next(PyRangeIter* iter, int64_t* out_value) {
    if (iter == NULL || iter->range == NULL) return 0;

    PyRange* range = iter->range;

    // Check if we're past the end
    if (range->step > 0) {
        if (iter->current >= range->stop) return 0;
    } else {
        if (iter->current <= range->stop) return 0;
    }

    // Store current value and advance
    *out_value = iter->current;
    iter->current += range->step;
    return 1;
}

// Get next value with default - compatible signature with iter_next_list
// Returns the next value, or default_value if exhausted
// Sets *exhausted to 1 if exhausted, 0 otherwise (pass NULL to ignore)
int64_t range_iter_next_default(PyRangeIter* iter, int64_t default_value, int8_t* exhausted) {
    if (exhausted) *exhausted = 0;
    if (iter == NULL || iter->range == NULL) {
        if (exhausted) *exhausted = 1;
        return default_value;
    }

    PyRange* range = iter->range;

    // Check if we're past the end
    if (range->step > 0) {
        if (iter->current >= range->stop) {
            if (exhausted) *exhausted = 1;
            return default_value;
        }
    } else {
        if (iter->current <= range->stop) {
            if (exhausted) *exhausted = 1;
            return default_value;
        }
    }

    // Store current value and advance
    int64_t result = iter->current;
    iter->current += range->step;
    return result;
}

// Free the iterator (does not free the range)
void range_iter_free(PyRangeIter* iter) {
    if (iter != NULL) {
        free(iter);
    }
}

// ============================================================================
// Range Utility Functions
// ============================================================================

// Get the start value
int64_t range_start(PyRange* range) {
    if (range == NULL) return 0;
    return range->start;
}

// Get the stop value
int64_t range_stop(PyRange* range) {
    if (range == NULL) return 0;
    return range->stop;
}

// Get the step value
int64_t range_step(PyRange* range) {
    if (range == NULL) return 0;
    return range->step;
}
