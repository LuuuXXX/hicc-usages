#include "variadic_functions.h"

int main() {
    std::cout << "sum_ints(3, 10, 20, 30) = "
              << variadic_ns::sum_ints(3, 10, 20, 30) << std::endl;
    variadic_ns::log_line("log: %s answer=%d", "demo", 42);
    std::cout << "sum_va_wrapper(4, 1, 2, 3, 4) = "
              << variadic_ns::sum_va_wrapper(4, 1, 2, 3, 4) << std::endl;
    return 0;
}
