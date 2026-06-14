#pragma once
#include <functional>
#include <string>
#include <memory>
#include <iostream>

namespace functional_bind_ns {

// Free function we will wrap with std::bind
int add(int a, int b);
int multiply(int a, int b);
int subtract(int a, int b);

// std::bind factories — produce std::function<int(int)>
// Each binds a fixed second argument and a placeholder for the first.
std::function<int(int)> make_adder(int n);
std::function<int(int)> make_multiplier(int n);
std::function<int(int)> make_subtractor(int n);

// Apply a bound function to a value
int apply_bound(std::function<int(int)> fn, int x);

// Compose two bound functions: outer(inner(x))
std::function<int(int)> compose(std::function<int(int)> outer, std::function<int(int)> inner);

// Class wrapping a bound function
class BoundAccumulator {
public:
    explicit BoundAccumulator(std::function<int(int)> fn) : fn_(std::move(fn)), base_(0) {}
    int call_and_accumulate(int x);          // calls fn_(x+base_) and adds result to base_
    int base() const { return base_; }
    void reset(int v) { base_ = v; }
private:
    std::function<int(int)> fn_;
    int base_;
};

std::unique_ptr<BoundAccumulator> make_accumulator(std::function<int(int)> fn);

} // namespace functional_bind_ns
