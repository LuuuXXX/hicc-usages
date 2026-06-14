#include "hicc_usages/template_instantiation.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::template_instantiation;
    Container<int>* ic = new Container<int>();
    ic->set(21);
    assert(ic->get() == 21);
    assert(ic->doubled() == 42);
    delete ic;

    Container<double>* dc = new Container<double>();
    dc->set(2.5);
    assert(dc->get() == 2.5);
    assert(dc->doubled() == 5.0);
    delete dc;
    std::cout << "[template_instantiation] C++ test OK" << std::endl;
    return 0;
}
