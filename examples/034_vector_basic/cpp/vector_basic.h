#pragma once
#include <vector>
#include <string>
#include <cstdint>
#include <iostream>

namespace vector_basic_ns {

// Demonstrates std::vector<int> usage.
long vector_sum(const std::vector<int>& v);
double vector_avg(const std::vector<int>& v);
std::vector<int> build_vector(int from, int to, int step);
void print_vector(const std::vector<int>& v);

} // namespace vector_basic_ns
