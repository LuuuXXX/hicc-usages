#pragma once
#include <functional>
#include <string>
#include <memory>
#include <iostream>

namespace std_function_ns {

// 可调用对象包装：存储一个 std::function，通过 operator()() 调用。
class Callback {
public:
    explicit Callback(std::function<int(int)> fn) : fn_(std::move(fn)) {}
    int invoke(int x) const { return fn_(x); }
    void replace(std::function<int(int)> fn) { fn_ = std::move(fn); }
    long call_n_times(int x, int n) const;  // 调用 fn_(x) n 次，返回总和
private:
    std::function<int(int)> fn_;
};

// 演示 std::function 作为参数 / 返回值的自由函数
int apply_dbl(std::function<int(int)> fn, int x);
std::function<int(int)> make_doubler();
int chain(std::function<int(int)> f, std::function<int(int)> g, int x);

std::unique_ptr<Callback> make_callback(std::function<int(int)> fn);

} // namespace std_function_ns
