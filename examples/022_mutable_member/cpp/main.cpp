#include "mutable_member.h"
#include <iostream>

int main() {
    Cache* c = cache_new();
    std::cout << "compute(5) = " << c->compute(5) << std::endl;
    std::cout << "compute(5) = " << c->compute(5) << std::endl;  // cached
    std::cout << "last = " << c->last_cached() << std::endl;
    cache_free(c);
    return 0;
}
