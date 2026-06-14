#include "vector_basic.h"

namespace vector_basic_ns {

long vector_sum(const std::vector<int>& v) {
    long s = 0;
    for (auto x : v) s += x;
    return s;
}

double vector_avg(const std::vector<int>& v) {
    if (v.empty()) return 0.0;
    return static_cast<double>(vector_sum(v)) / static_cast<double>(v.size());
}

std::vector<int> build_vector(int from, int to, int step) {
    std::vector<int> v;
    if (step == 0) return v;
    if (step > 0) {
        for (int i = from; i < to; i += step) v.push_back(i);
    } else {
        for (int i = from; i > to; i += step) v.push_back(i);
    }
    return v;
}

void print_vector(const std::vector<int>& v) {
    std::cout << "[";
    for (size_t i = 0; i < v.size(); ++i) {
        if (i) std::cout << ",";
        std::cout << v[i];
    }
    std::cout << "]" << std::endl;
}

int vector_basic_anchor() { return 34; }

} // namespace vector_basic_ns
