#include "hicc_usages/exception_basic.h"
#include <cassert>
#include <cstring>
#include <iostream>
int main() {
    using namespace hicc_usages::exception_basic;
    assert(safe_divide(10, 2) == 5);
    assert(safe_divide(10, 0) == 0);
    assert(strcmp(last_error(), "divide by zero") == 0);
    clear_error();

    int arr[] = {10, 20, 30};
    assert(safe_at(arr, 3, 1) == 20);
    assert(safe_at(arr, 3, 5) == -1);

    assert(safe_parse("123") == 123);
    assert(safe_parse("abc") == 0);

    SafeStack* s = SafeStack::create(2);
    s->push(1);
    s->push(2);
    s->push(3);  // overflow, sets error
    assert(strcmp(last_error(), "stack full") == 0);
    clear_error();
    assert(s->size() == 2);
    assert(s->pop() == 2);
    assert(s->pop() == 1);
    assert(s->pop() == -1);  // underflow
    SafeStack::free(s);
    std::cout << "[exception_basic] C++ test OK" << std::endl;
    return 0;
}
