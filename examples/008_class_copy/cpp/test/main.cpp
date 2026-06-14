#include "hicc_usages/class_copy.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::class_copy;
    Buffer* b1 = Buffer::create(10);
    b1->append(1); b1->append(2); b1->append(3);
    assert(b1->size() == 3);
    Buffer* b2 = Buffer::clone(b1);
    assert(b2->size() == 3);
    assert(b2->capacity() == 10);
    Buffer::free(b1); Buffer::free(b2);
    std::cout << "[class_copy] C++ test OK" << std::endl;
    return 0;
}
