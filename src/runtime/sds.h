// SDS (Simple Dynamic Strings) - Shared Header
// Based on Redis SDS library - BSD 3-Clause License
// Copyright (c) 2006-2015, Salvatore Sanfilippo <antirez at gmail dot com>

#ifndef SDS_H
#define SDS_H

#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <limits.h>

typedef char *sds;

#define SDS_MAX_PREALLOC (1024*1024)

#define SDS_TYPE_5  0
#define SDS_TYPE_8  1
#define SDS_TYPE_16 2
#define SDS_TYPE_32 3
#define SDS_TYPE_64 4
#define SDS_TYPE_MASK 7
#define SDS_TYPE_BITS 3

struct __attribute__ ((__packed__)) sdshdr5 {
    unsigned char flags;
    char buf[];
};

struct __attribute__ ((__packed__)) sdshdr8 {
    uint8_t len;
    uint8_t alloc;
    unsigned char flags;
    char buf[];
};

struct __attribute__ ((__packed__)) sdshdr16 {
    uint16_t len;
    uint16_t alloc;
    unsigned char flags;
    char buf[];
};

struct __attribute__ ((__packed__)) sdshdr32 {
    uint32_t len;
    uint32_t alloc;
    unsigned char flags;
    char buf[];
};

struct __attribute__ ((__packed__)) sdshdr64 {
    uint64_t len;
    uint64_t alloc;
    unsigned char flags;
    char buf[];
};

#define SDS_TYPE_5_LEN(f) ((f)>>SDS_TYPE_BITS)
#define SDS_HDR_VAR(T,s) struct sdshdr##T *sh = (void*)((s)-(sizeof(struct sdshdr##T)));
#define SDS_HDR(T,s) ((struct sdshdr##T *)((s)-(sizeof(struct sdshdr##T))))

static inline int sdsHdrSize(char type) {
    switch(type&SDS_TYPE_MASK) {
        case SDS_TYPE_5:  return sizeof(struct sdshdr5);
        case SDS_TYPE_8:  return sizeof(struct sdshdr8);
        case SDS_TYPE_16: return sizeof(struct sdshdr16);
        case SDS_TYPE_32: return sizeof(struct sdshdr32);
        case SDS_TYPE_64: return sizeof(struct sdshdr64);
    }
    return 0;
}

static inline char sdsReqType(size_t string_size) {
    if (string_size < 1<<5) return SDS_TYPE_5;
    if (string_size < 1<<8) return SDS_TYPE_8;
    if (string_size < 1<<16) return SDS_TYPE_16;
#if (LONG_MAX == LLONG_MAX)
    if (string_size < 1ll<<32) return SDS_TYPE_32;
    return SDS_TYPE_64;
#else
    return SDS_TYPE_32;
#endif
}

static inline size_t sdslen(const sds s) {
    unsigned char flags = s[-1];
    switch(flags&SDS_TYPE_MASK) {
        case SDS_TYPE_5: return SDS_TYPE_5_LEN(flags);
        case SDS_TYPE_8: return SDS_HDR(8,s)->len;
        case SDS_TYPE_16: return SDS_HDR(16,s)->len;
        case SDS_TYPE_32: return SDS_HDR(32,s)->len;
        case SDS_TYPE_64: return SDS_HDR(64,s)->len;
    }
    return 0;
}

static inline size_t sdsavail(const sds s) {
    unsigned char flags = s[-1];
    switch(flags&SDS_TYPE_MASK) {
        case SDS_TYPE_5: return 0;
        case SDS_TYPE_8: { SDS_HDR_VAR(8,s); return sh->alloc - sh->len; }
        case SDS_TYPE_16: { SDS_HDR_VAR(16,s); return sh->alloc - sh->len; }
        case SDS_TYPE_32: { SDS_HDR_VAR(32,s); return sh->alloc - sh->len; }
        case SDS_TYPE_64: { SDS_HDR_VAR(64,s); return sh->alloc - sh->len; }
    }
    return 0;
}

static inline void sdssetlen(sds s, size_t newlen) {
    unsigned char flags = s[-1];
    switch(flags&SDS_TYPE_MASK) {
        case SDS_TYPE_5: {
            unsigned char *fp = ((unsigned char*)s)-1;
            *fp = SDS_TYPE_5 | (newlen << SDS_TYPE_BITS);
            break;
        }
        case SDS_TYPE_8: SDS_HDR(8,s)->len = newlen; break;
        case SDS_TYPE_16: SDS_HDR(16,s)->len = newlen; break;
        case SDS_TYPE_32: SDS_HDR(32,s)->len = newlen; break;
        case SDS_TYPE_64: SDS_HDR(64,s)->len = newlen; break;
    }
}

static inline void sdsinclen(sds s, size_t inc) {
    unsigned char flags = s[-1];
    switch(flags&SDS_TYPE_MASK) {
        case SDS_TYPE_5: {
            unsigned char *fp = ((unsigned char*)s)-1;
            unsigned char newlen = SDS_TYPE_5_LEN(flags)+inc;
            *fp = SDS_TYPE_5 | (newlen << SDS_TYPE_BITS);
            break;
        }
        case SDS_TYPE_8: SDS_HDR(8,s)->len += inc; break;
        case SDS_TYPE_16: SDS_HDR(16,s)->len += inc; break;
        case SDS_TYPE_32: SDS_HDR(32,s)->len += inc; break;
        case SDS_TYPE_64: SDS_HDR(64,s)->len += inc; break;
    }
}

static inline size_t sdsalloc(const sds s) {
    unsigned char flags = s[-1];
    switch(flags&SDS_TYPE_MASK) {
        case SDS_TYPE_5: return SDS_TYPE_5_LEN(flags);
        case SDS_TYPE_8: return SDS_HDR(8,s)->alloc;
        case SDS_TYPE_16: return SDS_HDR(16,s)->alloc;
        case SDS_TYPE_32: return SDS_HDR(32,s)->alloc;
        case SDS_TYPE_64: return SDS_HDR(64,s)->alloc;
    }
    return 0;
}

static inline void sdssetalloc(sds s, size_t newlen) {
    unsigned char flags = s[-1];
    switch(flags&SDS_TYPE_MASK) {
        case SDS_TYPE_5: break;
        case SDS_TYPE_8: SDS_HDR(8,s)->alloc = newlen; break;
        case SDS_TYPE_16: SDS_HDR(16,s)->alloc = newlen; break;
        case SDS_TYPE_32: SDS_HDR(32,s)->alloc = newlen; break;
        case SDS_TYPE_64: SDS_HDR(64,s)->alloc = newlen; break;
    }
}

static inline sds sdsnewlen(const void *init, size_t initlen) {
    void *sh;
    sds s;
    char type = sdsReqType(initlen);
    if (type == SDS_TYPE_5 && initlen == 0) type = SDS_TYPE_8;
    int hdrlen = sdsHdrSize(type);
    unsigned char *fp;

    sh = malloc(hdrlen+initlen+1);
    if (sh == NULL) return NULL;
    if (!init) memset(sh, 0, hdrlen+initlen+1);
    s = (char*)sh+hdrlen;
    fp = ((unsigned char*)s)-1;
    switch(type) {
        case SDS_TYPE_5: *fp = type | (initlen << SDS_TYPE_BITS); break;
        case SDS_TYPE_8: { SDS_HDR_VAR(8,s); sh->len = initlen; sh->alloc = initlen; *fp = type; break; }
        case SDS_TYPE_16: { SDS_HDR_VAR(16,s); sh->len = initlen; sh->alloc = initlen; *fp = type; break; }
        case SDS_TYPE_32: { SDS_HDR_VAR(32,s); sh->len = initlen; sh->alloc = initlen; *fp = type; break; }
        case SDS_TYPE_64: { SDS_HDR_VAR(64,s); sh->len = initlen; sh->alloc = initlen; *fp = type; break; }
    }
    if (initlen && init) memcpy(s, init, initlen);
    s[initlen] = '\0';
    return s;
}

static inline sds sdsempty(void) { return sdsnewlen("", 0); }
static inline sds sdsnew(const char *init) { return sdsnewlen(init, init ? strlen(init) : 0); }
static inline sds sdsdup(const sds s) { return sdsnewlen(s, sdslen(s)); }

static inline void sdsfree(sds s) {
    if (s == NULL) return;
    free((char*)s-sdsHdrSize(s[-1]));
}

static inline sds sdsMakeRoomFor(sds s, size_t addlen) {
    void *sh, *newsh;
    size_t avail = sdsavail(s);
    size_t len, newlen;
    char type, oldtype = s[-1] & SDS_TYPE_MASK;
    int hdrlen;

    if (avail >= addlen) return s;
    len = sdslen(s);
    sh = (char*)s-sdsHdrSize(oldtype);
    newlen = len+addlen;
    if (newlen < SDS_MAX_PREALLOC) newlen *= 2;
    else newlen += SDS_MAX_PREALLOC;

    type = sdsReqType(newlen);
    if (type == SDS_TYPE_5) type = SDS_TYPE_8;
    hdrlen = sdsHdrSize(type);

    if (oldtype==type) {
        newsh = realloc(sh, hdrlen+newlen+1);
        if (newsh == NULL) return NULL;
        s = (char*)newsh+hdrlen;
    } else {
        newsh = malloc(hdrlen+newlen+1);
        if (newsh == NULL) return NULL;
        memcpy((char*)newsh+hdrlen, s, len+1);
        free(sh);
        s = (char*)newsh+hdrlen;
        s[-1] = type;
        sdssetlen(s, len);
    }
    sdssetalloc(s, newlen);
    return s;
}

static inline sds sdscatlen(sds s, const void *t, size_t len) {
    size_t curlen = sdslen(s);
    s = sdsMakeRoomFor(s, len);
    if (s == NULL) return NULL;
    memcpy(s+curlen, t, len);
    sdssetlen(s, curlen+len);
    s[curlen+len] = '\0';
    return s;
}

static inline sds sdscat(sds s, const char *t) { return sdscatlen(s, t, strlen(t)); }
static inline sds sdscatsds(sds s, const sds t) { return sdscatlen(s, t, sdslen(t)); }

static inline sds sdscpylen(sds s, const char *t, size_t len) {
    if (sdsalloc(s) < len) {
        s = sdsMakeRoomFor(s, len-sdslen(s));
        if (s == NULL) return NULL;
    }
    memcpy(s, t, len);
    s[len] = '\0';
    sdssetlen(s, len);
    return s;
}

static inline sds sdscpy(sds s, const char *t) { return sdscpylen(s, t, strlen(t)); }

static inline int sdscmp(const sds s1, const sds s2) {
    size_t l1 = sdslen(s1), l2 = sdslen(s2);
    size_t minlen = (l1 < l2) ? l1 : l2;
    int cmp = memcmp(s1, s2, minlen);
    if (cmp == 0) return l1>l2 ? 1 : (l1<l2 ? -1 : 0);
    return cmp;
}

static inline sds sdsgrowzero(sds s, size_t len) {
    size_t curlen = sdslen(s);
    if (len <= curlen) return s;
    s = sdsMakeRoomFor(s, len-curlen);
    if (s == NULL) return NULL;
    memset(s+curlen, 0, (len-curlen)+1);
    sdssetlen(s, len);
    return s;
}

static inline sds sdstrim(sds s, const char *cset) {
    char *sp, *ep;
    size_t len;
    sp = s;
    ep = s+sdslen(s)-1;
    while(sp <= ep && strchr(cset, *sp)) sp++;
    while(ep > sp && strchr(cset, *ep)) ep--;
    len = (sp > ep) ? 0 : ((ep-sp)+1);
    if (s != sp) memmove(s, sp, len);
    s[len] = '\0';
    sdssetlen(s, len);
    return s;
}

#endif // SDS_H
