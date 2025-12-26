#ifndef EXCEPTION_H
#define EXCEPTION_H

#include "types.h"
#include "str.h"

// ============================================================================
// Exception handling for Python-like try/except/finally
// Uses setjmp/longjmp for stack unwinding (freestanding implementation)
// ============================================================================

// Exception method macro
#define EXCEPTION_METHOD(name) BUILTIN_METHOD(Exception, name)

// ============================================================================
// Jump buffer for setjmp/longjmp
// Using __builtin_setjmp which stores 5 pointers
// ============================================================================

typedef struct {
    void* buf[5];  // __builtin_setjmp stores 5 pointers
} JmpBuf;

// ============================================================================
// Exception structure
// ============================================================================

typedef struct {
    String* type_name;    // Exception type name (e.g., "ValueError")
    String* message;      // Exception message
    String* parent_types; // Comma-separated parent type names (e.g., "BaseError,Exception")
} Exception;

// ============================================================================
// Exception frame for try block (linked list stack)
// ============================================================================

typedef struct ExceptionFrame {
    JmpBuf buf;                      // Jump buffer for longjmp
    struct ExceptionFrame* prev;     // Previous frame in stack
} ExceptionFrame;

// ============================================================================
// Low-level setjmp/longjmp (implemented in assembly)
// ============================================================================

// Save current execution context, returns 0 on direct call, non-zero on longjmp
int __pyc_setjmp(JmpBuf* buf);

// Restore execution context saved by setjmp, never returns
void __pyc_longjmp(JmpBuf* buf, int val) __attribute__((noreturn));

// ============================================================================
// Exception frame management
// ============================================================================

// Push a new exception frame onto the stack
void __pyc_push_exception_frame(ExceptionFrame* frame);

// Pop the current exception frame from the stack
void __pyc_pop_exception_frame(void);

// Get the current exception frame (for longjmp target)
ExceptionFrame* __pyc_get_exception_frame(void);

// ============================================================================
// Exception state management
// ============================================================================

// Get the current pending exception (NULL if none)
Exception* __pyc_get_exception(void);

// Set the current exception
void __pyc_set_exception(Exception* exc);

// Clear the current exception
void __pyc_clear_exception(void);

// Check if an exception is pending
int __pyc_has_exception(void);

// ============================================================================
// Raise an exception
// In polling mode: sets exception and returns (caller polls with __pyc_has_exception)
// If no handler, prints error and exits
// ============================================================================

void __pyc_raise(Exception* exc);

// Re-raise the current exception
// This exits if no exception is pending
void __pyc_reraise(void);

// ============================================================================
// Exception class methods
// ============================================================================

// Create a new exception: Exception(message)
Exception* EXCEPTION_METHOD(__init__)(String* message);

// Create a new exception with type name and parent types
Exception* __pyc_exception_new(String* type_name, String* message, String* parent_types);

// Exception.__str__()
String* EXCEPTION_METHOD(__str__)(Exception* exc);

// Exception.__repr__()
String* EXCEPTION_METHOD(__repr__)(Exception* exc);

// Get exception type name
String* __pyc_exception_type(Exception* exc);

// Check if exception matches a type (by name comparison)
int __pyc_exception_matches(Exception* exc, const char* type_name);

// Get the singleton StopIteration exception (avoids repeated allocations)
Exception* __pyc_stop_iteration(void);

#endif // EXCEPTION_H
