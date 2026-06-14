#pragma once
#include <cstddef>
#include <iostream>
namespace hicc_usages::template_function {

template<typename T>
T max_of(T a, T b) { return (a > b) ? a : b; }

template<typename T>
T min_of(T a, T b) { return (a < b) ? a : b; }

template<typename T>
T add_of(T a, T b) { return a + b; }

}  // namespace hicc_usages::template_function
