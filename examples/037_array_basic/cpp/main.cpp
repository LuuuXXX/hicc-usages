#include "array_basic.h"

int main() {
    using namespace array_basic_ns;
    std::array<int, 5> a{};
    fill_array(a, 10);  // [10, 11, 12, 13, 14]
    std::cout << "sum=" << array_sum(a) << std::endl;
    std::cout << "max=" << array_max(a) << std::endl;
    std::cout << "avg=" << array_avg(a) << std::endl;
    std::cout << "[";
    for (size_t i = 0; i < a.size(); ++i) {
        if (i) std::cout << ",";
        std::cout << a[i];
    }
    std::cout << "]" << std::endl;
    return 0;
}
