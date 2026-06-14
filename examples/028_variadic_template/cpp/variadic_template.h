#pragma once
#include <string>
#include <iostream>
#include <sstream>

namespace variadic_template_ns {

// 变参模板：递归终止
inline std::string format() { return ""; }

template <typename T, typename... Args>
std::string format(const T& first, const Args&... rest) {
    std::ostringstream oss;
    oss << first;
    return oss.str() + format(rest...);
}

// 变参模板：sum
inline int sum_all() { return 0; }

template <typename T, typename... Args>
int sum_all(const T& first, const Args&... rest) {
    return static_cast<int>(first) + sum_all(rest...);
}

// 显式实例化常见组合（不可能列全，主要走隐式实例化）
int sum_three(int a, int b, int c);

} // namespace variadic_template_ns
