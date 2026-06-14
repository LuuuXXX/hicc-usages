#pragma once
#include <functional>
#include <string>
#include <memory>
#include <iostream>

namespace std_function_ns {

// A callable wrapper: stores a std::function, calls it via operator()().
class Callback {
public:
    explicit Callback(std::function<int(int)> fn) : fn_(std::move(fn)) {}
    int invoke(int x) const { return fn_(x); }
    void replace(std::function<int(int)> fn) { fn_ = std::move(fn); }
    long call_n_times(int x, int n) const;  // calls fn_(x) n times, returns sum
private:
    std::function<int(int)> fn_;
};

// Free functions demonstrating std::function parameters / returns
int apply_dbl(std::function<int(int)> fn, int x);
std::function<int(int)> make_doubler();
int chain(std::function<int(int)> f, std::function<int(int)> g, int x);

std::unique_ptr<Callback> make_callback(std::function<int(int)> fn);

} // namespace std_function_ns
