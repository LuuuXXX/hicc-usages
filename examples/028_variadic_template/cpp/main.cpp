#include "variadic_template.h"

int main() {
    using namespace variadic_template_ns;
    std::cout << format("hello", ", ", "world", "!") << std::endl;
    std::cout << format(1, "+", 2, "=", 3) << std::endl;
    std::cout << "sum_all(1,2,3,4,5)=" << sum_all(1, 2, 3, 4, 5) << std::endl;
    std::cout << "sum_three(10,20,30)=" << sum_three(10, 20, 30) << std::endl;
    return 0;
}
