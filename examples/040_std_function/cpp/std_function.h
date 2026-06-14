#pragma once

#include <functional>

// ⚠️ std::function<int(int,int)> is not nameable across FFI.
// We expose named free functions that internally use std::function.

// Run an arbitrary binary int operation passed as a callable.
int run_binary_op(int a, int b, int op_kind);

// Pre-registered operations (defined in .cpp via std::function).
int add_op(int a, int b);
int mul_op(int a, int b);

// Compose two ops via std::function chaining.
int compose_then_add_then_mul(int x, int add_n, int mul_n);
