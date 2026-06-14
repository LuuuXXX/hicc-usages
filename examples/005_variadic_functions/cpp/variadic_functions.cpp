#include "variadic_functions.h"

int sum(int n, ...) {
    va_list args;
    va_start(args, n);
    int total = 0;
    for (int i = 0; i < n; ++i) total += va_arg(args, int);
    va_end(args);
    return total;
}

int sum2(int a, int b) { return a + b; }
int sum3(int a, int b, int c) { return a + b + c; }
