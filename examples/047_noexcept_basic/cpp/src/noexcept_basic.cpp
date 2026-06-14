#include "hicc_usages/noexcept_basic.h"
#include <cstdlib>
namespace hicc_usages::noexcept_basic {

int safe_add(int a, int b) noexcept { return a + b; }
int safe_sub(int a, int b) noexcept { return a - b; }
bool safe_equals(int a, int b) noexcept { return a == b; }
int no_throw_compute(int a, int b) noexcept {
    return a * b + a - b;
}
int maybe_throw_compute(int a, int b) {
    if (b < 0) return -1;
    return a / b;
}

NoexceptBuffer::NoexceptBuffer(std::size_t cap)
    : data_(static_cast<int*>(std::calloc(cap, sizeof(int)))), capacity_(cap) {}
NoexceptBuffer::~NoexceptBuffer() { std::free(data_); }
NoexceptBuffer* NoexceptBuffer::create(std::size_t capacity) {
    return new NoexceptBuffer(capacity);
}
void NoexceptBuffer::free(NoexceptBuffer* self) { delete self; }
void NoexceptBuffer::set(std::size_t idx, int v) {
    if (idx < capacity_) data_[idx] = v;
}
int NoexceptBuffer::get(std::size_t idx) const {
    return idx < capacity_ ? data_[idx] : 0;
}
std::size_t NoexceptBuffer::size() const { return capacity_; }
std::size_t NoexceptBuffer::capacity() const { return capacity_; }
void NoexceptBuffer::clear() {
    for (std::size_t i = 0; i < capacity_; ++i) data_[i] = 0;
}
int NoexceptBuffer::sum() const {
    int total = 0;
    for (std::size_t i = 0; i < capacity_; ++i) total += data_[i];
    return total;
}
}
