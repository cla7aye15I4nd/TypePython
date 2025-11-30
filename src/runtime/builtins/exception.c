// TypePython Runtime Library - Exception Module
// Exception handling support for try/except/finally/raise

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <setjmp.h>

// Exception type IDs - must match the Rust constants
#define EXC_BASE_EXCEPTION    0
#define EXC_EXCEPTION         1
#define EXC_VALUE_ERROR       2
#define EXC_TYPE_ERROR        3
#define EXC_KEY_ERROR         4
#define EXC_INDEX_ERROR       5
#define EXC_ZERO_DIVISION     6
#define EXC_RUNTIME_ERROR     7
#define EXC_STOP_ITERATION    8
#define EXC_ASSERTION_ERROR   9
#define EXC_GENERATOR_EXIT    10
#define EXC_ATTRIBUTE_ERROR   11
#define EXC_NAME_ERROR        12
#define EXC_OVERFLOW_ERROR    13
#define EXC_MEMORY_ERROR      14

// Exception object structure
typedef struct PyException {
    int64_t type_id;         // Exception type ID for fast matching
    char* type_name;         // Exception type name string
    char* message;           // Error message
    struct PyException* cause;  // Chained exception (__cause__)
    void* traceback;         // Future: traceback info
} PyException;

// Exception context for setjmp/longjmp based exception handling
// We use a simple stack-based approach for the stub implementation
#define MAX_EXCEPTION_HANDLERS 64

typedef struct {
    jmp_buf jump_buffer;
    PyException* current_exception;
    int64_t handler_type_ids[16];  // Types this handler catches
    int num_handler_types;
    int active;
} ExceptionHandler;

// Thread-local exception state (simplified - not actually thread-safe)
static ExceptionHandler exception_stack[MAX_EXCEPTION_HANDLERS];
static int exception_stack_top = 0;
static PyException* current_exception = NULL;

// ============================================================================
// Exception creation and destruction
// ============================================================================

PyException* exception_new(int64_t type_id, const char* type_name, const char* message) {
    PyException* exc = (PyException*)malloc(sizeof(PyException));
    if (!exc) return NULL;

    exc->type_id = type_id;
    exc->type_name = type_name ? strdup(type_name) : NULL;
    exc->message = message ? strdup(message) : NULL;
    exc->cause = NULL;
    exc->traceback = NULL;

    return exc;
}

PyException* exception_new_with_cause(int64_t type_id, const char* type_name,
                                       const char* message, PyException* cause) {
    PyException* exc = exception_new(type_id, type_name, message);
    if (exc) {
        exc->cause = cause;
    }
    return exc;
}

void exception_free(PyException* exc) {
    if (!exc) return;

    if (exc->type_name) free(exc->type_name);
    if (exc->message) free(exc->message);
    // Note: We don't free cause here - ownership is complex
    free(exc);
}

// ============================================================================
// Exception info accessors
// ============================================================================

int64_t exception_type_id(PyException* exc) {
    return exc ? exc->type_id : -1;
}

const char* exception_type_name(PyException* exc) {
    return exc ? exc->type_name : "Unknown";
}

const char* exception_message(PyException* exc) {
    return exc ? exc->message : "";
}

PyException* exception_cause(PyException* exc) {
    return exc ? exc->cause : NULL;
}

// ============================================================================
// Exception type matching
// ============================================================================

// Check if exception is an instance of the given type (or subtype)
// This is simplified - doesn't handle full inheritance hierarchy
int64_t exception_isinstance(PyException* exc, int64_t type_id) {
    if (!exc) return 0;

    // Exact match
    if (exc->type_id == type_id) return 1;

    // BaseException catches everything
    if (type_id == EXC_BASE_EXCEPTION) return 1;

    // Exception catches all standard exceptions (not BaseException)
    if (type_id == EXC_EXCEPTION && exc->type_id != EXC_BASE_EXCEPTION) return 1;

    return 0;
}

// Get type ID from type name string
int64_t exception_type_id_from_name(const char* name) {
    if (!name) return EXC_EXCEPTION;

    if (strcmp(name, "BaseException") == 0) return EXC_BASE_EXCEPTION;
    if (strcmp(name, "Exception") == 0) return EXC_EXCEPTION;
    if (strcmp(name, "ValueError") == 0) return EXC_VALUE_ERROR;
    if (strcmp(name, "TypeError") == 0) return EXC_TYPE_ERROR;
    if (strcmp(name, "KeyError") == 0) return EXC_KEY_ERROR;
    if (strcmp(name, "IndexError") == 0) return EXC_INDEX_ERROR;
    if (strcmp(name, "ZeroDivisionError") == 0) return EXC_ZERO_DIVISION;
    if (strcmp(name, "RuntimeError") == 0) return EXC_RUNTIME_ERROR;
    if (strcmp(name, "StopIteration") == 0) return EXC_STOP_ITERATION;
    if (strcmp(name, "AssertionError") == 0) return EXC_ASSERTION_ERROR;
    if (strcmp(name, "GeneratorExit") == 0) return EXC_GENERATOR_EXIT;
    if (strcmp(name, "AttributeError") == 0) return EXC_ATTRIBUTE_ERROR;
    if (strcmp(name, "NameError") == 0) return EXC_NAME_ERROR;
    if (strcmp(name, "OverflowError") == 0) return EXC_OVERFLOW_ERROR;
    if (strcmp(name, "MemoryError") == 0) return EXC_MEMORY_ERROR;

    // Default to generic Exception
    return EXC_EXCEPTION;
}

// Get type name from type ID
const char* exception_name_from_type_id(int64_t type_id) {
    switch (type_id) {
        case EXC_BASE_EXCEPTION: return "BaseException";
        case EXC_EXCEPTION: return "Exception";
        case EXC_VALUE_ERROR: return "ValueError";
        case EXC_TYPE_ERROR: return "TypeError";
        case EXC_KEY_ERROR: return "KeyError";
        case EXC_INDEX_ERROR: return "IndexError";
        case EXC_ZERO_DIVISION: return "ZeroDivisionError";
        case EXC_RUNTIME_ERROR: return "RuntimeError";
        case EXC_STOP_ITERATION: return "StopIteration";
        case EXC_ASSERTION_ERROR: return "AssertionError";
        case EXC_GENERATOR_EXIT: return "GeneratorExit";
        case EXC_ATTRIBUTE_ERROR: return "AttributeError";
        case EXC_NAME_ERROR: return "NameError";
        case EXC_OVERFLOW_ERROR: return "OverflowError";
        case EXC_MEMORY_ERROR: return "MemoryError";
        default: return "Exception";
    }
}

// ============================================================================
// Exception handling (setjmp/longjmp based - simple stub implementation)
// ============================================================================

// Push a new exception handler onto the stack
// Returns pointer to jump buffer that should be used with setjmp
void* exception_push_handler(void) {
    if (exception_stack_top >= MAX_EXCEPTION_HANDLERS) {
        fprintf(stderr, "Exception handler stack overflow\n");
        abort();
    }

    ExceptionHandler* handler = &exception_stack[exception_stack_top++];
    handler->current_exception = NULL;
    handler->num_handler_types = 0;
    handler->active = 1;

    return &handler->jump_buffer;
}

// Pop the current exception handler
void exception_pop_handler(void) {
    if (exception_stack_top > 0) {
        exception_stack[--exception_stack_top].active = 0;
    }
}

// Raise an exception - longjmp to nearest handler
void exception_raise(PyException* exc) {
    current_exception = exc;

    // Find matching handler
    for (int i = exception_stack_top - 1; i >= 0; i--) {
        if (exception_stack[i].active) {
            exception_stack[i].current_exception = exc;
            longjmp(exception_stack[i].jump_buffer, 1);
        }
    }

    // No handler found - print and abort
    fprintf(stderr, "Unhandled exception: %s",
            exc ? exception_type_name(exc) : "Unknown");
    if (exc && exc->message && exc->message[0]) {
        fprintf(stderr, ": %s", exc->message);
    }
    fprintf(stderr, "\n");
    abort();
}

// Re-raise the current exception
void exception_reraise(void) {
    if (current_exception) {
        exception_raise(current_exception);
    }
}

// Get the current exception (for except block binding)
PyException* exception_current(void) {
    return current_exception;
}

// Clear the current exception
void exception_clear(void) {
    current_exception = NULL;
}

// ============================================================================
// Convenience functions for common exception types
// ============================================================================

void raise_value_error(const char* message) {
    exception_raise(exception_new(EXC_VALUE_ERROR, "ValueError", message));
}

void raise_type_error(const char* message) {
    exception_raise(exception_new(EXC_TYPE_ERROR, "TypeError", message));
}

void raise_key_error(const char* message) {
    exception_raise(exception_new(EXC_KEY_ERROR, "KeyError", message));
}

void raise_index_error(const char* message) {
    exception_raise(exception_new(EXC_INDEX_ERROR, "IndexError", message));
}

void raise_zero_division_error(const char* message) {
    exception_raise(exception_new(EXC_ZERO_DIVISION, "ZeroDivisionError", message));
}

void raise_runtime_error(const char* message) {
    exception_raise(exception_new(EXC_RUNTIME_ERROR, "RuntimeError", message));
}

void raise_assertion_error(const char* message) {
    exception_raise(exception_new(EXC_ASSERTION_ERROR, "AssertionError", message));
}

void raise_stop_iteration(void) {
    exception_raise(exception_new(EXC_STOP_ITERATION, "StopIteration", NULL));
}

// Print exception info for debugging
void exception_print(PyException* exc) {
    if (!exc) {
        printf("None\n");
        return;
    }

    printf("%s", exception_type_name(exc));
    if (exc->message && exc->message[0]) {
        printf(": %s", exc->message);
    }
    printf("\n");

    if (exc->cause) {
        printf("\nThe above exception was the direct cause of:\n\n");
        exception_print(exc->cause);
    }
}
