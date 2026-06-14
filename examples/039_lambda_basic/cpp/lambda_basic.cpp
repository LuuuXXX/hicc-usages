#include "lambda_basic.h"

int double_it(int x) { return x * 2; }

int add_then_double(int a, int b) { return (a + b) * 2; }

int sum_with_offset(int* arr, int n, int offset) {
    int total = offset;
    for (int i = 0; i < n; ++i) total += arr[i];
    return total;
}
