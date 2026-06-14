#include "virtual_diamond.h"

Console* console_new()      { return new Console(); }
void     console_free(Console* c) { delete c; }
