#include "hicc_usages/noexcept_basic.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::noexcept_basic;
    static_assert(noexcept(safe_add(1, 2)), "safe_add should be noexcept");
    static_assert(noexcept(safe_sub(1, 2)), "safe_sub should be noexcept");

    assert(safe_add(3, 4) == 7);
    assert(safe_sub(10, 4) == 6);
    assert(safe_equals(5, 5));
    assert(!safe_equals(5, 6));
    assert(no_throw_compute(3, 4) == 11);
    assert(maybe_throw_compute(10, 2) == 5);

    NoexceptBuffer* b = NoexceptBuffer::create(4);
    assert(b->capacity() == 4);
    b->set(0, 10);
    b->set(1, 20);
    b->set(2, 30);
    b->set(3, 40);
    assert(b->get(1) == 20);
    assert(b->sum() == 100);
    b->clear();
    assert(b->sum() == 0);
    NoexceptBuffer::free(b);
    std::cout << "[noexcept_basic] C++ test OK" << std::endl;
    return 0;
}
