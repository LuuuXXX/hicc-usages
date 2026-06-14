#pragma once
#include <string>
#include <memory>
#include <iostream>

namespace noexcept_basic_ns {

// noexcept 自由函数
int add_noexcept(int a, int b) noexcept;
int square_noexcept(int x) noexcept;
double safe_reciprocal_noexcept(double x) noexcept;  // x == 0 时返回 0
constexpr int compute_constant() noexcept { return 42; }

// 一个不是 noexcept 的函数（用于对比）
int may_throw(int x);

// 带 noexcept 方法的类
class SafeCounter {
public:
    SafeCounter() noexcept;
    void increment(int by) noexcept;
    int get() const noexcept;
    void reset() noexcept;
    std::string describe() const noexcept;  // std::string 在内存压力下可能抛异常，但我们标 noexcept
private:
    int value_;
};

std::unique_ptr<SafeCounter> make_counter() noexcept;

// 带 noexcept 移动构造的 move-only 类型
class Buffer {
public:
    explicit Buffer(size_t n) noexcept;
    Buffer(Buffer&& other) noexcept;
    Buffer& operator=(Buffer&& other) noexcept;
    size_t size() const noexcept;
    int get(size_t idx) const noexcept;
    void set(size_t idx, int value) noexcept;
private:
    int* data_;
    size_t size_;
};

std::unique_ptr<Buffer> make_buffer(size_t n) noexcept;

} // namespace noexcept_basic_ns
