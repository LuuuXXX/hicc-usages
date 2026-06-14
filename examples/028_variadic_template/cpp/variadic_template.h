#pragma once

// Variadic template (parameter pack). hicc can't bind `T...` directly — we
// provide fixed-arity wrappers (sum2, sum3, sum4) that delegate to the
// variadic template.

template <typename... Ts>
int sum_all(Ts... args) {
    return (0 + ... + args);  // fold expression
}

// Fixed-arity wrappers — these are what Rust binds to.
inline int sum2(int a, int b)                { return sum_all(a, b); }
inline int sum3(int a, int b, int c)         { return sum_all(a, b, c); }
inline int sum4(int a, int b, int c, int d)  { return sum_all(a, b, c, d); }
