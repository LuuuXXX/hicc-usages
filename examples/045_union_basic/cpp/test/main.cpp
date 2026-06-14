#include "hicc_usages/union_basic.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::union_basic;
    // 直接测试联合体（untouched C++ mode: 包装由 rust_gen 在 Rust 侧自动生成）
    Value v;
    v.as_int = 42;
    assert(v.as_int == 42);
    v.as_double = 3.14;
    assert(v.as_double == 3.14);
    v.as_long = 1234567890L;
    assert(v.as_long == 1234567890L);

    Pair* p = Pair::create(5, 9);
    assert(p->first() == 5);
    assert(p->second() == 9);
    assert(p->sum() == 14);
    assert(p->max() == 9);
    Pair::free(p);
    std::cout << "[union_basic] C++ test OK" << std::endl;
    return 0;
}
