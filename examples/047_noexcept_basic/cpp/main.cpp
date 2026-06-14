#include "noexcept_basic.h"
#include <iostream>

int main() {
    using namespace noexcept_basic_ns;
    std::cout << "add_noexcept(2, 3) = " << add_noexcept(2, 3) << std::endl;
    std::cout << "square_noexcept(7) = " << square_noexcept(7) << std::endl;
    std::cout << "safe_reciprocal(4) = " << safe_reciprocal_noexcept(4.0) << std::endl;
    std::cout << "safe_reciprocal(0) = " << safe_reciprocal_noexcept(0.0) << std::endl;
    std::cout << "compute_constant() = " << compute_constant() << std::endl;

    auto counter = make_counter();
    counter->increment(5);
    counter->increment(3);
    std::cout << "counter.get() = " << counter->get() << std::endl;
    std::cout << "counter.describe() = " << counter->describe() << std::endl;

    auto buf = make_buffer(4);
    buf->set(0, 10);
    buf->set(3, 40);
    std::cout << "buf.size() = " << buf->size() << std::endl;
    std::cout << "buf.get(0) = " << buf->get(0) << std::endl;
    std::cout << "buf.get(3) = " << buf->get(3) << std::endl;
    return 0;
}
