// TypePython Runtime Library - Generator Module
// Generator and iterator support for yield/yield from

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

// Generator state values
#define GEN_CREATED    0    // Generator created, not yet started
#define GEN_RUNNING    1    // Generator is currently executing
#define GEN_SUSPENDED  2    // Generator suspended at yield point
#define GEN_CLOSED     3    // Generator has finished or been closed

// Generator object structure
typedef struct PyGenerator {
    int64_t state;              // Current state (GEN_CREATED, GEN_SUSPENDED, etc.)
    int64_t yield_point;        // Which yield point to resume at (0 = start)
    void* frame;                // Pointer to saved local variables
    int64_t frame_size;         // Size of frame in bytes
    void* resume_fn;            // Function pointer to resume function
    void* exception;            // Pending exception (for throw)
    int64_t has_value;          // Whether last yield produced a value
    int64_t yielded_value;      // Last yielded value (simplified - just int64)
    int64_t sent_value;         // Value sent via send() (simplified - just int64)
} PyGenerator;

// ============================================================================
// Generator creation and destruction
// ============================================================================

PyGenerator* generator_new(void* resume_fn, int64_t frame_size) {
    PyGenerator* gen = (PyGenerator*)malloc(sizeof(PyGenerator));
    if (!gen) return NULL;

    gen->state = GEN_CREATED;
    gen->yield_point = 0;
    gen->frame = frame_size > 0 ? calloc(1, frame_size) : NULL;
    gen->frame_size = frame_size;
    gen->resume_fn = resume_fn;
    gen->exception = NULL;
    gen->has_value = 0;
    gen->yielded_value = 0;
    gen->sent_value = 0;

    return gen;
}

void generator_free(PyGenerator* gen) {
    if (!gen) return;

    if (gen->frame) free(gen->frame);
    free(gen);
}

// ============================================================================
// Generator state accessors
// ============================================================================

int64_t generator_state(PyGenerator* gen) {
    return gen ? gen->state : GEN_CLOSED;
}

void* generator_frame(PyGenerator* gen) {
    return gen ? gen->frame : NULL;
}

int64_t generator_yield_point(PyGenerator* gen) {
    return gen ? gen->yield_point : -1;
}

// Set yield point and mark as suspended
void generator_set_yield_point(PyGenerator* gen, int64_t point) {
    if (gen) {
        gen->yield_point = point;
        gen->state = GEN_SUSPENDED;
    }
}

// Mark generator as closed
void generator_set_closed(PyGenerator* gen) {
    if (gen) {
        gen->state = GEN_CLOSED;
    }
}

// ============================================================================
// Generator iteration protocol
// ============================================================================

// Start or resume generator, returns 1 if yielded value, 0 if done
// The resume function should:
// 1. Switch on gen->yield_point to jump to correct location
// 2. Execute code until yield or return
// 3. If yielding: store value in gen->yielded_value, call generator_set_yield_point, return 1
// 4. If returning: call generator_set_closed, return 0
//
// This function is called by the compiler-generated __next__ implementation
int64_t generator_next(PyGenerator* gen, int64_t* out_value) {
    if (!gen) {
        if (out_value) *out_value = 0;
        return 0;
    }

    // Check if generator is exhausted
    if (gen->state == GEN_CLOSED) {
        if (out_value) *out_value = 0;
        return 0; // StopIteration
    }

    // Set state to running
    gen->state = GEN_RUNNING;
    gen->sent_value = 0; // Default sent value is None (0)

    // Call the resume function
    // The resume function will update gen->state, gen->yield_point, gen->yielded_value
    typedef int64_t (*ResumeFn)(PyGenerator*);
    ResumeFn resume = (ResumeFn)gen->resume_fn;

    int64_t has_value = resume(gen);

    if (has_value && out_value) {
        *out_value = gen->yielded_value;
    }

    return has_value;
}

// Send a value into the generator
int64_t generator_send(PyGenerator* gen, int64_t value, int64_t* out_value) {
    if (!gen) {
        if (out_value) *out_value = 0;
        return 0;
    }

    // Cannot send to a created generator (must use next() first)
    if (gen->state == GEN_CREATED && value != 0) {
        // TypeError: can't send non-None value to a just-started generator
        if (out_value) *out_value = 0;
        return 0;
    }

    // Check if generator is exhausted
    if (gen->state == GEN_CLOSED) {
        if (out_value) *out_value = 0;
        return 0; // StopIteration
    }

    // Set the sent value and resume
    gen->sent_value = value;
    gen->state = GEN_RUNNING;

    typedef int64_t (*ResumeFn)(PyGenerator*);
    ResumeFn resume = (ResumeFn)gen->resume_fn;

    int64_t has_value = resume(gen);

    if (has_value && out_value) {
        *out_value = gen->yielded_value;
    }

    return has_value;
}

// Throw an exception into the generator
int64_t generator_throw(PyGenerator* gen, void* exc, int64_t* out_value) {
    if (!gen) {
        if (out_value) *out_value = 0;
        return 0;
    }

    // Cannot throw into a closed generator
    if (gen->state == GEN_CLOSED) {
        if (out_value) *out_value = 0;
        return 0;
    }

    gen->exception = exc;
    gen->state = GEN_RUNNING;

    typedef int64_t (*ResumeFn)(PyGenerator*);
    ResumeFn resume = (ResumeFn)gen->resume_fn;

    int64_t has_value = resume(gen);

    gen->exception = NULL;

    if (has_value && out_value) {
        *out_value = gen->yielded_value;
    }

    return has_value;
}

// Close the generator
void generator_close(PyGenerator* gen) {
    if (!gen) return;

    if (gen->state == GEN_CLOSED) return;

    // If generator hasn't started or is already suspended, just close it
    if (gen->state == GEN_CREATED || gen->state == GEN_SUSPENDED) {
        gen->state = GEN_CLOSED;
        return;
    }

    // Otherwise, send GeneratorExit and ignore the result
    gen->state = GEN_CLOSED;
}

// ============================================================================
// Yield value storage (called by generated code)
// ============================================================================

void generator_yield_value(PyGenerator* gen, int64_t value) {
    if (gen) {
        gen->yielded_value = value;
        gen->has_value = 1;
    }
}

// Get sent value (called by generated code after yield expression)
int64_t generator_get_sent_value(PyGenerator* gen) {
    return gen ? gen->sent_value : 0;
}

// Get pending exception (called by generated code)
void* generator_get_exception(PyGenerator* gen) {
    return gen ? gen->exception : NULL;
}

// ============================================================================
// Iterator protocol helpers
// ============================================================================

// Create an iterator from various sources
// Returns a generator-like object that can be iterated

// For range: we already have range_iterator in range.c

// Check if an object is a generator
int64_t is_generator(void* obj) {
    // Simple check - in real implementation would check type tag
    return obj != NULL;
}

// ============================================================================
// Debug helpers
// ============================================================================

void generator_print_state(PyGenerator* gen) {
    if (!gen) {
        printf("Generator: NULL\n");
        return;
    }

    const char* state_str;
    switch (gen->state) {
        case GEN_CREATED: state_str = "created"; break;
        case GEN_RUNNING: state_str = "running"; break;
        case GEN_SUSPENDED: state_str = "suspended"; break;
        case GEN_CLOSED: state_str = "closed"; break;
        default: state_str = "unknown"; break;
    }

    printf("Generator { state: %s, yield_point: %ld, has_value: %ld, value: %ld }\n",
           state_str, (long)gen->yield_point, (long)gen->has_value, (long)gen->yielded_value);
}
