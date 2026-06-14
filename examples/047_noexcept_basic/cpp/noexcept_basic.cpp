#include "noexcept_basic.h"

int SafeAdder::add(int x) const { return base + x; }
int SafeAdder::sub(int x) const { return base - x; }
int SafeAdder::combined(int x, int y) const { return base + x - y; }

SafeAdder* safe_adder_new(int base) { return new SafeAdder{base}; }
void safe_adder_free(SafeAdder* s) { delete s; }
