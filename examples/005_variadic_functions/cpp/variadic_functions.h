#pragma once

#include <cstdarg>

// C-style variadic — not directly FFI-safe, so we also provide fixed-arity
// wrappers (sum2 / sum3) which are what Rust binds to.
int sum(int n, ...);
int sum2(int a, int b);
int sum3(int a, int b, int c);
