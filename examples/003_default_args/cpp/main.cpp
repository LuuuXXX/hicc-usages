#include "default_args.h"

int main() {
    default_args_ns::greet("Alice");
    default_args_ns::greet("Bob", 2);
    default_args_ns::greet("Carol", 3, "?");
    std::cout << "compute(1) = " << default_args_ns::compute(1) << std::endl;
    std::cout << "compute(1, 2) = " << default_args_ns::compute(1, 2) << std::endl;
    std::cout << "compute(1, 2, 3) = " << default_args_ns::compute(1, 2, 3) << std::endl;
    return 0;
}
