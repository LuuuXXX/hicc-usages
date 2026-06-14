#include "array_basic.h"

namespace array_basic_ns {

long array_sum(const std::array<int, 5>& a) {
    long s = 0;
    for (auto x : a) s += x;
    return s;
}

int array_max(const std::array<int, 5>& a) {
    int m = a[0];
    for (size_t i = 1; i < a.size(); ++i) {
        if (a[i] > m) m = a[i];
    }
    return m;
}

double array_avg(const std::array<int, 5>& a) {
    return static_cast<double>(array_sum(a)) / static_cast<double>(a.size());
}

void fill_array(std::array<int, 5>& a, int start) {
    for (size_t i = 0; i < a.size(); ++i) a[i] = start + static_cast<int>(i);
}

int array_basic_anchor() { return 37; }

} // namespace array_basic_ns
