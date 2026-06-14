#include "mutable_member.h"

Cache* cache_new()   { return new Cache(); }
void   cache_free(Cache* c) { delete c; }
