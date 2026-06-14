#include "template_specialization.h"

int main() {
    using namespace template_specialization_ns;
    std::cout << "TypeInfo<int>::name=" << TypeInfo<int>::name() << std::endl;
    std::cout << TypeInfo<int>::describe(42) << std::endl;
    std::cout << TypeInfo<double>::describe(3.14) << std::endl;
    std::cout << TypeInfo<std::string>::describe("hi") << std::endl;
    return 0;
}
