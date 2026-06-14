#include "template_function.h"

// Explicit instantiations — emitted as callable symbols.
template int    identity<int>(int);
template double identity<double>(double);
template int    add_tmpl<int>(int, int);
template double add_tmpl<double>(double, double);
