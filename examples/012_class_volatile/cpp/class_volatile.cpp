#include "class_volatile.h"

VCounter* vcounter_new()         { return new VCounter(); }
void      vcounter_free(VCounter* c) { delete c; }
