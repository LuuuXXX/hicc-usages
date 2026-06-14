#include "vector_basic.h"

IntVector* int_vec_new() { return new IntVector(); }
void       int_vec_free(IntVector* v) { delete v; }
