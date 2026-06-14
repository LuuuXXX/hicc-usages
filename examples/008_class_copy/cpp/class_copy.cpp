#include "class_copy.h"

Box* box_new(int v)           { return new Box(v); }
Box* box_clone(const Box* s)  { return new Box(*s); }   // copy ctor
void box_free(Box* b)         { delete b; }
