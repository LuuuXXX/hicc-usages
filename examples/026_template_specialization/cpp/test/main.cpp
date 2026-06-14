#include "hicc_usages/template_specialization.h"
#include <cassert>
#include <cstring>
#include <iostream>
int main() {
    using namespace hicc_usages::template_specialization;
    assert(std::strcmp(TypeInfo<int>::name(), "int") == 0);
    assert(std::strcmp(TypeInfo<double>::name(), "double") == 0);
    assert(std::strcmp(TypeInfo<char>::name(), "char") == 0);
    assert(TypeInfo<int>::size_of() == 4);
    assert(TypeInfo<double>::size_of() == 8);
    std::cout << "[template_specialization] C++ test OK" << std::endl;
    return 0;
}
