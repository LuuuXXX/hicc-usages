#include "hicc_usages/namespace_nested.h"
namespace hicc_usages::namespace_nested {

namespace outer {
    namespace inner {
        int add(int a, int b) { return a + b; }
        int multiply(int a, int b) { return a * b; }

        Calculator* Calculator::create() { return new Calculator(); }
        void Calculator::free(Calculator* self) { delete self; }
        int Calculator::compute(int a, int b) const { return add(a, b) + multiply(a, b); }
    }

    int subtract(int a, int b) { return a - b; }

    Helper* Helper::create() { return new Helper(); }
    void Helper::free(Helper* self) { delete self; }
    int Helper::doubled(int x) const { return x * 2; }
}

int outer_inner_sum(int a, int b, int c) {
    return outer::inner::add(a, b) + outer::subtract(c, 0);
}
}
