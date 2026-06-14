#include "noexcept_basic.h"
#include <iostream>

int main() {
    SafeAdder* s = safe_adder_new(100);
    std::cout << "add(5) = " << s->add(5) << std::endl;
    std::cout << "sub(5) = " << s->sub(5) << std::endl;
    std::cout << "combined(7,3) = " << s->combined(7, 3) << std::endl;
    safe_adder_free(s);
    return 0;
}
