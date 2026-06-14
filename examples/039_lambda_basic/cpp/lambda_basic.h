#pragma once
#include <functional>
#include <string>
#include <iostream>

namespace lambda_basic_ns {

// 调用 fn(x) —— 从 Rust 端传一个 lambda（以 std::function 形式）到 C++。
int apply_int(int x, std::function<int(int)> fn);

// 在 C++ 中构造一个 lambda，返回为 std::function。
// 捕获 `add`，返回 [add](int v) { return v + add; }
std::function<int(int)> make_adder(int add);

// 组合两个 int->int 函数。
std::function<int(int)> compose(std::function<int(int)> f, std::function<int(int)> g);

// 高阶函数：接收一个 string -> string 的 lambda
std::string shout(std::function<std::string(std::string)> fn, const std::string& input);

} // namespace lambda_basic_ns
