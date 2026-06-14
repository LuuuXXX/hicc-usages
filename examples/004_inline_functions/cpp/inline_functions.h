#pragma once

// inline functions are typically defined in the header itself — the compiler
// emits a weak symbol that's safe to link across translation units.
inline int square(int x) { return x * x; }
inline int cube(int x) { return x * x * x; }
