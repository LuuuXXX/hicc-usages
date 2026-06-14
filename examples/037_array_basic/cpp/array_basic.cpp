#include "array_basic.h"

IntArray4* int_array4_new() { return new IntArray4(); }
void       int_array4_free(IntArray4* a) { delete a; }
