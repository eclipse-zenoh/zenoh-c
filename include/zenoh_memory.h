#pragma once
#include <stdlib.h>

/*------------------ Memory ------------------*/
static inline void *z_malloc(size_t size) { return malloc(size); }
static inline void *z_realloc(void *ptr, size_t size) { return realloc(ptr, size); }
static inline void z_free(void *ptr) { free(ptr); }