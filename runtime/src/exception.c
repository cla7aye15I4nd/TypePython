#include "exception.h"
#include "runtime.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

// ============================================================================
// Global exception state
// ============================================================================

static ExceptionFrame* current_frame = NULL;
static Exception* current_exception = NULL;
static Exception* stop_iteration_singleton = NULL;

// ============================================================================
// Stubs for setjmp/longjmp (polling-based, no actual jumps)
// ============================================================================

int __pyc_setjmp(JmpBuf* buf) {
    (void)buf;
    return 0;
}

void __pyc_longjmp(JmpBuf* buf, int val) {
    (void)buf;
    (void)val;
}

// ============================================================================
// Exception frame management
// ============================================================================

void __pyc_push_exception_frame(ExceptionFrame* frame) {
    frame->prev = current_frame;
    current_frame = frame;
}

void __pyc_pop_exception_frame(void) {
    if (current_frame) {
        current_frame = current_frame->prev;
    }
}

ExceptionFrame* __pyc_get_exception_frame(void) {
    return current_frame;
}

// ============================================================================
// Exception state management
// ============================================================================

Exception* __pyc_get_exception(void) {
    return current_exception;
}

void __pyc_set_exception(Exception* exc) {
    current_exception = exc;
}

void __pyc_clear_exception(void) {
    current_exception = NULL;
}

int __pyc_has_exception(void) {
    return current_exception != NULL;
}

// ============================================================================
// Raise exception
// ============================================================================

void __pyc_raise(Exception* exc) {
    current_exception = exc;

    if (!current_frame) {
        // No handler - print error and exit
        fputs("Uncaught exception", stderr);
        if (exc && exc->type_name) {
            fputs(": ", stderr);
            fwrite(exc->type_name->data, 1, exc->type_name->len, stderr);
        }
        if (exc && exc->message && exc->message->len > 0) {
            fputs(": ", stderr);
            fwrite(exc->message->data, 1, exc->message->len, stderr);
        }
        fputc('\n', stderr);
        exit(1);
    }
}

void __pyc_reraise(void) {
    if (current_exception) {
        __pyc_raise(current_exception);
    } else {
        fputs("RuntimeError: No active exception to re-raise\n", stderr);
        exit(1);
    }
}

// ============================================================================
// Exception class methods
// ============================================================================

Exception* EXCEPTION_METHOD(__init__)(String* message) {
    return __pyc_exception_new(STR_METHOD(from_literal)("Exception", 9), message, NULL);
}

Exception* __pyc_exception_new(String* type_name, String* message, String* parent_types) {
    Exception* exc = (Exception*)malloc(sizeof(Exception));
    exc->type_name = type_name;
    exc->message = message;
    exc->parent_types = parent_types;
    return exc;
}

String* EXCEPTION_METHOD(__str__)(Exception* exc) {
    if (exc && exc->message) {
        return exc->message;
    }
    return STR_METHOD(from_literal)("", 0);
}

String* EXCEPTION_METHOD(__repr__)(Exception* exc) {
    if (!exc) {
        return STR_METHOD(from_literal)("Exception()", 11);
    }

    int64_t type_len = exc->type_name ? exc->type_name->len : 9;
    int64_t msg_len = exc->message ? exc->message->len : 0;
    int64_t total_len = type_len + 4 + msg_len;  // "Type('msg')"

    String* result = (String*)malloc(sizeof(String) + total_len + 1);
    result->len = total_len;

    char* p = result->data;

    if (exc->type_name) {
        memcpy(p, exc->type_name->data, exc->type_name->len);
        p += exc->type_name->len;
    } else {
        memcpy(p, "Exception", 9);
        p += 9;
    }

    *p++ = '(';
    *p++ = '\'';

    if (exc->message && exc->message->len > 0) {
        memcpy(p, exc->message->data, exc->message->len);
        p += exc->message->len;
    }

    *p++ = '\'';
    *p++ = ')';
    *p = '\0';

    return result;
}

String* __pyc_exception_type(Exception* exc) {
    if (exc && exc->type_name) {
        return exc->type_name;
    }
    return STR_METHOD(from_literal)("Exception", 9);
}

Exception* __pyc_stop_iteration(void) {
    if (!stop_iteration_singleton) {
        stop_iteration_singleton = __pyc_exception_new(
            STR_METHOD(from_literal)("StopIteration", 13),
            STR_METHOD(from_literal)("", 0),
            NULL
        );
    }
    return stop_iteration_singleton;
}

int __pyc_exception_matches(Exception* exc, const char* type_name) {
    if (!exc || !exc->type_name || !type_name) {
        return 0;
    }

    // "Exception" is the base class and matches all exceptions
    if (strcmp(type_name, "Exception") == 0) {
        return 1;
    }

    // Exact match for specific exception types
    if (strcmp(exc->type_name->data, type_name) == 0) {
        return 1;
    }

    // Check parent types (comma-separated list like "MiddleError,BaseError,Exception")
    if (exc->parent_types && exc->parent_types->len > 0) {
        const char* parents = exc->parent_types->data;
        size_t type_len = strlen(type_name);
        const char* p = parents;
        const char* end = parents + exc->parent_types->len;

        while (p < end) {
            // Find end of current parent name (comma or end of string)
            const char* comma = p;
            while (comma < end && *comma != ',') {
                comma++;
            }
            size_t parent_len = comma - p;

            // Compare with type_name
            if (parent_len == type_len && memcmp(p, type_name, type_len) == 0) {
                return 1;
            }

            // Move to next parent (skip comma)
            p = comma < end ? comma + 1 : end;
        }
    }

    return 0;
}
