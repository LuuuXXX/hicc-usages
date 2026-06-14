#pragma once
#include <functional>
#include <string>
#include <iostream>

namespace lambda_basic_ns {

// Apply fn(x) — pass a lambda (as std::function) from Rust to C++.
int apply_int(int x, std::function<int(int)> fn);

// Build a lambda in C++, return as std::function.
// Captures `add`, returning [add](int v) { return v + add; }
std::function<int(int)> make_adder(int add);

// Compose two int->int functions.
std::function<int(int)> compose(std::function<int(int)> f, std::function<int(int)> g);

// Higher-order: take a string -> string lambda
std::string shout(std::function<std::string(std::string)> fn, const std::string& input);

} // namespace lambda_basic_ns
