#pragma once
#include <string>
#include <iostream>

namespace overload_ns {

inline int add(int a, int b) { return a + b; }
inline double add(double a, double b) { return a + b; }
inline std::string add(const std::string& a, const std::string& b) { return a + b; }

inline int add(int a, int b, int c) { return a + b + c; }

} // namespace overload_ns
