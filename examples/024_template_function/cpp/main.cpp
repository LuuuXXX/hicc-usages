#include "template_function.h"
#include <iostream>

int main() {
    std::cout << "identity<int>(7) = "    << identity<int>(7)    << std::endl;
    std::cout << "identity<double>(2.5) = " << identity<double>(2.5) << std::endl;
    std::cout << "add<int>(2,3) = "       << add_tmpl<int>(2, 3) << std::endl;
    std::cout << "add<double>(1.5,2.5) = " << add_tmpl<double>(1.5, 2.5) << std::endl;
    return 0;
}
