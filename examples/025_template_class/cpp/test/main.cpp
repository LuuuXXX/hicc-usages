#include "hicc_usages/template_class.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::template_class;
    Stack<int>* s = new Stack<int>();
    s->push(10);
    s->push(20);
    assert(s->size() == 2);
    assert(s->top() == 20);
    assert(s->pop() == 20);
    assert(s->pop() == 10);
    assert(s->size() == 0);
    delete s;
    std::cout << "[template_class] C++ test OK" << std::endl;
    return 0;
}
