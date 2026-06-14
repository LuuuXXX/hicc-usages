#include "std_function.h"
#include <iostream>

int main() {
    std::cout << "add_op(2,3) = " << add_op(2, 3) << std::endl;
    std::cout << "run_binary_op(2,3,0) = " << run_binary_op(2, 3, 0) << std::endl;
    std::cout << "run_binary_op(2,3,1) = " << run_binary_op(2, 3, 1) << std::endl;
    std::cout << "compose_then_add_then_mul(2,10,3) = "
              << compose_then_add_then_mul(2, 10, 3) << std::endl;
    return 0;
}
