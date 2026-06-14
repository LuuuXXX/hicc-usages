#include "exception_basic.h"

int safe_divide(int a, int b) {
    if (b == 0) return 0;
    return a / b;
}

int throwing_divide(int a, int b) {
    if (b == 0) throw std::runtime_error("divide by zero");
    return a / b;
}

int checked_index(const int* arr, int n, int i) {
    if (i < 0 || i >= n) throw std::out_of_range("index out of range");
    return arr[i];
}
