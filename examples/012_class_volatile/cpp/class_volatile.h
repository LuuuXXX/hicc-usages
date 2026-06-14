#pragma once
#include <iostream>
#include <string>

namespace class_volatile_ns {

// volatile 主要用于硬件寄存器、信号处理器等场景。
// Rust 没有 volatile 概念，需通过包装函数擦除 volatile 后才能 FFI。

class Sensor {
public:
    Sensor(int id) : id_(id), reading_(0), counter_(0) {}

    // volatile 成员函数
    int read() volatile const { return reading_; }
    void write(int v) volatile { reading_ = v; ++counter_; }

    int id() const { return id_; }
    int counter() const { return counter_; }

    // 非 volatile 桥接（供 Rust 调用）
    int safe_read() const { return const_cast<const int&>(reading_); }
    void safe_write(int v) { reading_ = v; ++counter_; }

private:
    int id_;
    mutable volatile int reading_;
    mutable int counter_;
};

} // namespace class_volatile_ns
