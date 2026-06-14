#pragma once
#include <cstddef>
#include <iostream>
namespace hicc_usages::namespace_nested {

namespace outer {
    namespace inner {
        int add(int a, int b);
        int multiply(int a, int b);

        class Calculator {
        public:
            static Calculator* create();
            static void free(Calculator* self);
            int compute(int a, int b) const;
        };
    }

    int subtract(int a, int b);

    class Helper {
    public:
        static Helper* create();
        static void free(Helper* self);
        int doubled(int x) const;
    };
}

int outer_inner_sum(int a, int b, int c);

}  // namespace hicc_usages::namespace_nested
