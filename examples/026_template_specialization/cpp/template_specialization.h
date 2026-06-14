#pragma once
#include <string>
#include <iostream>

namespace template_specialization_ns {

// 通用模板
template <typename T>
struct TypeInfo {
    static const char* name() { return "unknown"; }
    static std::string describe(const T& v) { return "value=" + std::to_string(v); }
};

// 偏特化：int
template <>
struct TypeInfo<int> {
    static const char* name() { return "int"; }
    static std::string describe(const int& v) { return "int(" + std::to_string(v) + ")"; }
};

// 偏特化：double
template <>
struct TypeInfo<double> {
    static const char* name() { return "double"; }
    static std::string describe(const double& v) { return "double(" + std::to_string(v) + ")"; }
};

// 偏特化：std::string
template <>
struct TypeInfo<std::string> {
    static const char* name() { return "string"; }
    static std::string describe(const std::string& v) { return "string(" + v + ")"; }
};

} // namespace template_specialization_ns
