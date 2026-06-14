#include "exception_basic.h"
#include <iostream>

int main() {
    std::cout << "safe_divide(10,2) = " << safe_divide(10, 2) << std::endl;

    try {
        throwing_divide(10, 0);
    } catch (const std::exception& e) {
        std::cout << "caught: " << e.what() << std::endl;
    }

    int arr[] = {1, 2, 3};
    try {
        checked_index(arr, 3, 5);
    } catch (const std::exception& e) {
        std::cout << "caught: " << e.what() << std::endl;
    }
    return 0;
}
