#include "vector_basic.h"

int main() {
    using namespace vector_basic_ns;
    auto v = build_vector(1, 11, 2);  // [1,3,5,7,9]
    print_vector(v);
    std::cout << "sum=" << vector_sum(v) << std::endl;
    std::cout << "avg=" << vector_avg(v) << std::endl;
    return 0;
}
