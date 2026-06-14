#include "hicc_usages/vector_basic.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::vector_basic;
    IntVector* v = IntVector::create();
    v->push_back(10);
    v->push_back(20);
    v->push_back(30);
    assert(v->size() == 3);
    assert(v->at(0) == 10);
    assert(v->sum() == 60);
    v->pop_back();
    assert(v->size() == 2);
    v->clear();
    assert(v->size() == 0);
    IntVector::free(v);
    std::cout << "[vector_basic] C++ test OK" << std::endl;
    return 0;
}
