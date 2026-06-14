#pragma once
#include <cstdarg>
#include <cstdio>
#include <string>
#include <iostream>

namespace variadic_ns {

// C 风格变长参数
inline int sum_ints(int count, ...) {
    va_list args;
    va_start(args, count);
    int total = 0;
    for (int i = 0; i < count; ++i) {
        total += va_arg(args, int);
    }
    va_end(args);
    return total;
}

// printf 风格
inline void log_line(const char* fmt, ...) {
    va_list args;
    va_start(args, fmt);
    vprintf(fmt, args);
    va_end(args);
    putchar('\n');
}

// va_list 风格（hicc 友好：last param 是 va_list）
inline int sum_va(int count, va_list args) {
    int total = 0;
    for (int i = 0; i < count; ++i) {
        total += va_arg(args, int);
    }
    return total;
}

inline int sum_va_wrapper(int count, ...) {
    va_list args;
    va_start(args, count);
    int r = sum_va(count, args);
    va_end(args);
    return r;
}

} // namespace variadic_ns
