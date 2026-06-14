#pragma once

// ⚠️ Lambda types are not nameable across FFI. We write named free functions
// in C++ that capture the same closure logic.

int double_it(int x);        // was: auto f = [](int x) { return x * 2; };
int add_then_double(int a, int b);  // was: auto g = [](int a, int b) { return (a+b)*2; };
int sum_with_offset(int* arr, int n, int offset);  // captures `offset`
