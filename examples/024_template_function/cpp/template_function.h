#pragma once
#include <string>
#include <iostream>
#include <vector>

namespace template_function_ns {

// 模板函数：泛型加法
template <typename T>
T add(const T& a, const T& b) {
    return a + b;
}

// 模板函数：max
template <typename T>
const T& max_of(const T& a, const T& b) {
    return a < b ? b : a;
}

// 模板函数：to_string 风格
template <typename T>
std::string describe(const T& v) {
    return "value=" + std::to_string(v);
}

// 显式实例化（确保 FFI 能找到符号）
template int add<int>(const int&, const int&);
template double add<double>(const double&, const double&);
template const int& max_of<int>(const int&, const int&);
template std::string describe<int>(const int&);

} // namespace template_function_ns
