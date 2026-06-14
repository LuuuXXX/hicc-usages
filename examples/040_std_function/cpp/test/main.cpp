#include "hicc_usages/std_function.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::std_function;
    FuncStore* fs = FuncStore::create();
    assert(!fs->has_func());
    fs->set_adder();
    assert(fs->call(5) == 15);
    fs->set_multiplier();
    assert(fs->call(4) == 12);
    fs->set_constant(99);
    assert(fs->call(100) == 99);
    FuncStore::free(fs);

    Dispatcher* d = Dispatcher::create();
    d->set_mode(0);
    assert(d->run(5) == 5);
    d->set_mode(1);
    assert(d->run(5) == 6);
    assert(d->run_twice(5) == 7);
    d->set_mode(2);
    assert(d->run_twice(3) == 12);
    Dispatcher::free(d);
    std::cout << "[std_function] C++ test OK" << std::endl;
    return 0;
}
