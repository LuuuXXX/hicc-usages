#include "std_function.h"

int add_op(int a, int b) { return a + b; }
int mul_op(int a, int b) { return a * b; }

int run_binary_op(int a, int b, int op_kind) {
    // Internally dispatch through std::function — opaque to FFI.
    std::function<int(int, int)> f;
    switch (op_kind) {
        case 0: f = add_op; break;
        case 1: f = mul_op; break;
        default: f = [](int x, int y) { return x - y; }; break;
    }
    return f(a, b);
}

int compose_then_add_then_mul(int x, int add_n, int mul_n) {
    std::function<int(int)> step1 = [add_n](int v) { return v + add_n; };
    std::function<int(int)> step2 = [mul_n](int v) { return v * mul_n; };
    std::function<int(int)> composed = [&](int v) { return step2(step1(v)); };
    return composed(x);
}
