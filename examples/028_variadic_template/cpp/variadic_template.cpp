#include "variadic_template.h"

namespace variadic_template_ns {

// 用模板实现具体签名的桥接函数
int sum_three(int a, int b, int c) {
    return sum_all(a, b, c);
}

int variadic_template_anchor() { return 28; }
}
