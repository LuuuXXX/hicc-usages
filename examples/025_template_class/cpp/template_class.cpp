#include "template_class.h"

// Explicit instantiation — produces the symbols Rust will reach through.
template class BoxT<int>;
template class BoxT<double>;
