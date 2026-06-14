#include "functional_bind.h"
#include <functional>

namespace functional_bind_ns {

int add(int a, int b) { return a + b; }
int multiply(int a, int b) { return a * b; }
int subtract(int a, int b) { return a - b; }

std::function<int(int)> make_adder(int n) {
    return std::bind(add, std::placeholders::_1, n);
}

std::function<int(int)> make_multiplier(int n) {
    return std::bind(multiply, std::placeholders::_1, n);
}

std::function<int(int)> make_subtractor(int n) {
    return std::bind(subtract, std::placeholders::_1, n);
}

int apply_bound(std::function<int(int)> fn, int x) {
    return fn(x);
}

std::function<int(int)> compose(std::function<int(int)> outer, std::function<int(int)> inner) {
    return [outer, inner](int x) { return outer(inner(x)); };
}

int BoundAccumulator::call_and_accumulate(int x) {
    int r = fn_(base_ + x);
    base_ += r;
    return r;
}

std::unique_ptr<BoundAccumulator> make_accumulator(std::function<int(int)> fn) {
    return std::unique_ptr<BoundAccumulator>(new BoundAccumulator(std::move(fn)));
}

int functional_bind_anchor() { return 41; }

} // namespace functional_bind_ns
