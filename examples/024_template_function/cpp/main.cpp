#include "template_function.h"

int main() {
    using namespace template_function_ns;
    std::cout << "add<int>(2,3)=" << add<int>(2, 3) << std::endl;
    std::cout << "add<double>(2.5,3.5)=" << add<double>(2.5, 3.5) << std::endl;
    std::cout << "max_of<int>(7,3)=" << max_of<int>(7, 3) << std::endl;
    std::cout << "describe<int>(42)=" << describe<int>(42) << std::endl;
    return 0;
}
