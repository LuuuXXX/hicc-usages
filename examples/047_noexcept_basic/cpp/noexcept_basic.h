#pragma once
#include <string>
#include <memory>
#include <iostream>

namespace noexcept_basic_ns {

// noexcept free functions
int add_noexcept(int a, int b) noexcept;
int square_noexcept(int x) noexcept;
double safe_reciprocal_noexcept(double x) noexcept;  // returns 0 if x == 0
constexpr int compute_constant() noexcept { return 42; }

// A function that is NOT noexcept (for contrast)
int may_throw(int x);

// Class with noexcept methods
class SafeCounter {
public:
    SafeCounter() noexcept;
    void increment(int by) noexcept;
    int get() const noexcept;
    void reset() noexcept;
    std::string describe() const noexcept;  // std::string may throw under memory pressure, but we mark noexcept
private:
    int value_;
};

std::unique_ptr<SafeCounter> make_counter() noexcept;

// Move-only type with noexcept move ctor
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
