#include "tuple_basic.h"
#include <iostream>

int main() {
    Triple* t = triple_new(42, "hello", 3.14);
    std::cout << "first="  << t->first()
              << " second=" << t->second()
              << " third="  << t->third() << std::endl;
    triple_free(t);
    return 0;
}
