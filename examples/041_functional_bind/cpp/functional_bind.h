#pragma once
#include <functional>
#include <string>
#include <memory>
#include <iostream>

namespace functional_bind_ns {

// 将用 std::bind 包装的自由函数
int add(int a, int b);
int multiply(int a, int b);
int subtract(int a, int b);

// std::bind 工厂 —— 产出 std::function<int(int)>
// 每个绑定固定的第二个参数，第一个用占位符。
std::function<int(int)> make_adder(int n);
std::function<int(int)> make_multiplier(int n);
std::function<int(int)> make_subtractor(int n);

// 把已绑定函数应用到某个值
int apply_bound(std::function<int(int)> fn, int x);

// 组合两个已绑定函数：outer(inner(x))
std::function<int(int)> compose(std::function<int(int)> outer, std::function<int(int)> inner);

// 包装已绑定函数的类
class BoundAccumulator {
public:
    explicit BoundAccumulator(std::function<int(int)> fn) : fn_(std::move(fn)), base_(0) {}
    int call_and_accumulate(int x);          // 调用 fn_(x+base_) 并把结果累加进 base_
    int base() const { return base_; }
    void reset(int v) { base_ = v; }
private:
    std::function<int(int)> fn_;
    int base_;
};

std::unique_ptr<BoundAccumulator> make_accumulator(std::function<int(int)> fn);

} // namespace functional_bind_ns
