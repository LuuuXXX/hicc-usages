#include "class_move.h"

Resource* resource_new(int v)   { return new Resource(v); }
void      resource_free(Resource* r) { delete r; }
