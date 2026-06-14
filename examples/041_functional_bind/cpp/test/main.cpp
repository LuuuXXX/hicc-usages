#include "hicc_usages/functional_bind.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::functional_bind;
    Binder* b = Binder::create();
    b->configure(10, 5);
    assert(b->next() == 10);
    assert(b->next() == 15);
    assert(b->next() == 20);
    assert(b->calls() == 3);
    assert(b->peek() == 25);
    Binder::free(b);

    Combiner* c = Combiner::create();
    c->set_first(double_it);
    c->set_pipeline();
    assert(c->process(7) == 28);  // double_it(7) = 14, *2 = 28
    Combiner::free(c);

    assert(double_it(5) == 10);
    assert(add_one(5) == 6);
    std::cout << "[functional_bind] C++ test OK" << std::endl;
    return 0;
}
