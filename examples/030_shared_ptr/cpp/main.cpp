#include "shared_ptr.h"
#include <iostream>

int main() {
    auto a = make_shared_obj();
    auto b = a;  // shared ownership
    std::cout << "use_count = " << a.use_count() << std::endl;
    std::cout << "total = " << shared_count() << std::endl;
    return 0;
}
