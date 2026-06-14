#include "class_basic.h"

Counter* counter_new()        { return new Counter(); }
void     counter_free(Counter* c) { delete c; }
