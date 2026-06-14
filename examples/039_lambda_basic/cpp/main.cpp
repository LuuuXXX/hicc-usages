#include "lambda_basic.h"
#include <iostream>

int main() {
    std::cout << "double_it(21) = " << double_it(21) << std::endl;
    std::cout << "add_then_double(2,3) = " << add_then_double(2, 3) << std::endl;
    int arr[] = {1, 2, 3};
    std::cout << "sum_with_offset(arr,3,10) = " << sum_with_offset(arr, 3, 10) << std::endl;
    return 0;
}
