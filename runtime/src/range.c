#include "runtime.h"
#include <stdlib.h>
#include <stdio.h>

Range* __pyc___builtin___range_1(int64_t stop) {
    Range* r = (Range*)malloc(sizeof(Range));
    if (r == NULL) {
        rt_panic("Failed to allocate memory for range");
    }
    r->start = 0;
    r->stop = stop;
    r->step = 1;
    r->current = 0;
    return r;
}

Range* __pyc___builtin___range_2(int64_t start, int64_t stop) {
    Range* r = (Range*)malloc(sizeof(Range));
    if (r == NULL) {
        rt_panic("Failed to allocate memory for range");
    }
    r->start = start;
    r->stop = stop;
    r->step = 1;
    r->current = start;
    return r;
}

Range* __pyc___builtin___range_3(int64_t start, int64_t stop, int64_t step) {
    if (step == 0) {
        rt_panic("range() step argument must not be zero");
    }
    Range* r = (Range*)malloc(sizeof(Range));
    if (r == NULL) {
        rt_panic("Failed to allocate memory for range");
    }
    r->start = start;
    r->stop = stop;
    r->step = step;
    r->current = start;
    return r;
}

Range* RANGE_METHOD(__iter__)(Range* r) {
    if (r != NULL) {
        r->current = r->start;
    }
    return r;
}

int64_t RANGE_METHOD(__next__)(Range* r) {
    if (r == NULL) {
        rt_panic("Cannot iterate over NULL range");
    }

    int done = (r->step > 0) ? (r->current >= r->stop) : (r->current <= r->stop);
    if (done) {
        __pyc_raise(__pyc_stop_iteration());
        return 0;
    }

    int64_t result = r->current;
    r->current += r->step;
    return result;
}

void RANGE_METHOD(__dealloc__)(Range* r) {
    free(r);
}

int64_t RANGE_METHOD(__len__)(Range* r) {
    if (r == NULL) return 0;

    if (r->step > 0) {
        if (r->start >= r->stop) return 0;
        return (r->stop - r->start + r->step - 1) / r->step;
    } else {
        if (r->start <= r->stop) return 0;
        return (r->start - r->stop - r->step - 1) / (-r->step);
    }
}

String* RANGE_METHOD(__str__)(Range* r) {
    if (r == NULL) {
        return STR_METHOD(from_literal)("range()", 7);
    }

    char buffer[128];
    int len;
    if (r->step != 1) {
        len = snprintf(buffer, sizeof(buffer), "range(%ld, %ld, %ld)", r->start, r->stop, r->step);
    } else {
        len = snprintf(buffer, sizeof(buffer), "range(%ld, %ld)", r->start, r->stop);
    }

    return STR_METHOD(from_literal)(buffer, len);
}

String* RANGE_METHOD(__repr__)(Range* r) {
    return RANGE_METHOD(__str__)(r);
}
