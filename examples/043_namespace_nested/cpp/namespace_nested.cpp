#include "namespace_nested.h"

namespace outer::inner::core {
    int add(int a, int b) { return a + b; }
    int mul(int a, int b) { return a * b; }
}

namespace outer::inner::util {
    int combined(int a, int b, int c) {
        using namespace outer::inner::core;
        return add(add(a, b), c);
    }
}

int ns_add(int a, int b) { return outer::inner::core::add(a, b); }
int ns_mul(int a, int b) { return outer::inner::core::mul(a, b); }
int ns_combined(int a, int b, int c) { return outer::inner::util::combined(a, b, c); }
