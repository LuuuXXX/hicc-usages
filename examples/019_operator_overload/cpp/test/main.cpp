#include "hicc_usages/operator_overload.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::operator_overload;
    Vec2* a = Vec2::create(1, 2);
    Vec2* b = Vec2::create(3, 4);
    Vec2 a_ref = *a;
    Vec2 b_ref = *b;
    assert(a->dot(b_ref) == 11);
    Vec2 sum = *a + *b;
    assert(sum.get_x() == 4 && sum.get_y() == 6);
    Vec2 diff = *b - *a;
    assert(diff.get_x() == 2 && diff.get_y() == 2);
    Vec2 scaled = *a * 3;
    assert(scaled.get_x() == 3 && scaled.get_y() == 6);
    assert(!(*a == *b));
    Vec2::free(a); Vec2::free(b);
    std::cout << "[operator_overload] C++ test OK" << std::endl;
    return 0;
}
