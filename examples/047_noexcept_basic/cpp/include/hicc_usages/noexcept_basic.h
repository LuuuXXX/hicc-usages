#pragma once
#include <cstddef>
#include <iostream>
namespace hicc_usages::noexcept_basic {

int safe_add(int a, int b) noexcept;
int safe_sub(int a, int b) noexcept;
bool safe_equals(int a, int b) noexcept;
int no_throw_compute(int a, int b) noexcept;
int maybe_throw_compute(int a, int b);

class NoexceptBuffer {
public:
    static NoexceptBuffer* create(std::size_t capacity);
    static void free(NoexceptBuffer* self);
    void set(std::size_t idx, int v);
    int get(std::size_t idx) const;
    std::size_t size() const;
    std::size_t capacity() const;
    void clear();
    int sum() const;
private:
    NoexceptBuffer(std::size_t cap);
    ~NoexceptBuffer();
    int* data_;
    std::size_t capacity_;
};

}  // namespace hicc_usages::noexcept_basic
