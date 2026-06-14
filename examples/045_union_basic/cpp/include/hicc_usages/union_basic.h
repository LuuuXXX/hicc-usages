#pragma once
#include <cstddef>
#include <iostream>
namespace hicc_usages::union_basic {

// 原始联合体 —— 不提供任何包装，rust_gen 自动生成 ValueBox shim
union Value {
    int as_int;
    double as_double;
    long as_long;
};

class Pair {
public:
    static Pair* create(int first, int second);
    static void free(Pair* self);
    int first() const;
    int second() const;
    int sum() const;
    int max() const;
private:
    Pair(int a, int b);
    ~Pair();
    int first_;
    int second_;
};

}  // namespace hicc_usages::union_basic
