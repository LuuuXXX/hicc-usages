#include "map_basic.h"

StringIntMap* str_int_map_new() { return new StringIntMap(); }
void          str_int_map_free(StringIntMap* m) { delete m; }
