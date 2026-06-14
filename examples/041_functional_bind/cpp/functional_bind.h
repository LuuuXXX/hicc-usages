#pragma once

// ⚠️ Result of std::bind is not nameable across FFI.
// Named wrappers encapsulate the bind expression.

struct BindPoint { int x; int y; };

BindPoint* bind_point_new(int x, int y);
void bind_point_free(BindPoint* p);

int add_bound_10(int x);
int mul_bound_3(int x);
int sub_bind_first(int a, int b);
int point_x_plus_offset(const BindPoint* p, int offset);
