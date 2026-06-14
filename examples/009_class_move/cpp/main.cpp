#include "class_move.h"
#include <iostream>
#include <utility>

int main() {
    Resource* r = resource_new(99);
    std::cout << "peek=" << r->peek() << " valid=" << r->is_valid() << std::endl;

    // Move ctor: takes ownership of *r's internals.
    Resource moved = std::move(*r);
    int v = std::move(moved).consume_value();
    std::cout << "consumed=" << v << std::endl;
    std::cout << "moved-from r valid=" << r->is_valid() << std::endl;

    resource_free(r);
    return 0;
}
