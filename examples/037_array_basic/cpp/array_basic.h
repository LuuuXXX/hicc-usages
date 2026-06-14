#pragma once
#include <array>
#include <cstdint>
#include <iostream>

namespace array_basic_ns {

// 演示 std::array<int, 5> 的用法。
long array_sum(const std::array<int, 5>& a);
int array_max(const std::array<int, 5>& a);
double array_avg(const std::array<int, 5>& a);
void fill_array(std::array<int, 5>& a, int start);  // a[i] = start + i

} // namespace array_basic_ns
