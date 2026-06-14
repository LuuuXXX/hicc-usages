#pragma once

#include <stdexcept>

// hicc translates C++ throw/catch into Exception<T> on the Rust side
// (no exception types cross the FFI; only success value + what() string).

// Safe: returns a plain int.
int safe_divide(int a, int b);

// Throws std::runtime_error when b == 0.
// FFI returns hicc::Exception<int>; the caller checks via .ok().
int throwing_divide(int a, int b);

// Throws std::out_of_range; shows different exception type mapping to same channel.
int checked_index(const int* arr, int n, int i);
