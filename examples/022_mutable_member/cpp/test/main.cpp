#include "hicc_usages/mutable_member.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::mutable_member;
    Cache* c = Cache::create();
    c->set_value(0, 42);
    const Cache* cc = c;
    assert(cc->get_value(0) == 42);
    assert(cc->access_count() == 1);
    Cache::free(c);
    std::cout << "[mutable_member] C++ test OK" << std::endl;
    return 0;
}
