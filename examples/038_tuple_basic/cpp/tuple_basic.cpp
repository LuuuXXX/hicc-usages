#include "tuple_basic.h"

Triple* triple_new(int i, const char* s, double d) { return new Triple(i, s, d); }
void     triple_free(Triple* t) { delete t; }
