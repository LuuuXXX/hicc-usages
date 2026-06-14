#include "hicc_usages/map_basic.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::map_basic;
    IntMap* m = IntMap::create();
    m->put(1, 100);
    m->put(2, 200);
    m->put(3, 300);
    assert(m->size() == 3);
    assert(m->has(2));
    assert(m->get(2) == 200);
    assert(!m->has(99));
    m->erase(2);
    assert(!m->has(2));
    assert(m->sum_values() == 400);
    IntMap::free(m);
    std::cout << "[map_basic] C++ test OK" << std::endl;
    return 0;
}
