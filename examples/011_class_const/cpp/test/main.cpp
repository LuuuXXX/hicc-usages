#include "hicc_usages/class_const.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::class_const;
    const Value* v = Value::create(10);
    assert(v->get() == 10);
    Value* mut = Value::create(5);
    mut->add(3);
    assert(mut->get() == 8);
    mut->set(100);
    assert(mut->get() == 100);
    Value::free(const_cast<Value*>(v)); Value::free(mut);
    std::cout << "[class_const] C++ test OK" << std::endl;
    return 0;
}
