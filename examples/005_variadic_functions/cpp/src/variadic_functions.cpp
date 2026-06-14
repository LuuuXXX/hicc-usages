#include "hicc_usages/variadic_functions.h"
namespace hicc_usages::variadic_functions {
int sum_2(int a, int b) { return a + b; }
int sum_3(int a, int b, int c) { return a + b + c; }
int sum_4(int a, int b, int c, int d) { return a + b + c + d; }
int sum_array(const int* arr, int count) {
    int s = 0; for (int i = 0; i < count; ++i) s += arr[i]; return s;
}
}
