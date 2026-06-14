#include "noexcept_basic.h"

namespace noexcept_basic_ns {

int add_noexcept(int a, int b) noexcept { return a + b; }
int square_noexcept(int x) noexcept { return x * x; }
double safe_reciprocal_noexcept(double x) noexcept {
    if (x == 0.0) return 0.0;
    return 1.0 / x;
}

int may_throw(int x) {
    if (x < 0) throw std::runtime_error("negative");
    return x * 2;
}

SafeCounter::SafeCounter() noexcept : value_(0) {}
void SafeCounter::increment(int by) noexcept { value_ += by; }
int SafeCounter::get() const noexcept { return value_; }
void SafeCounter::reset() noexcept { value_ = 0; }
std::string SafeCounter::describe() const noexcept {
    return "SafeCounter(" + std::to_string(value_) + ")";
}

std::unique_ptr<SafeCounter> make_counter() noexcept {
    return std::unique_ptr<SafeCounter>(new SafeCounter());
}

Buffer::Buffer(size_t n) noexcept : data_(nullptr), size_(n) {
    if (n > 0) data_ = new (std::nothrow) int[n]();
}
Buffer::Buffer(Buffer&& other) noexcept : data_(other.data_), size_(other.size_) {
    other.data_ = nullptr;
    other.size_ = 0;
}
Buffer& Buffer::operator=(Buffer&& other) noexcept {
    if (this != &other) {
        delete[] data_;
        data_ = other.data_;
        size_ = other.size_;
        other.data_ = nullptr;
        other.size_ = 0;
    }
    return *this;
}
size_t Buffer::size() const noexcept { return size_; }
int Buffer::get(size_t idx) const noexcept {
    if (idx >= size_ || !data_) return 0;
    return data_[idx];
}
void Buffer::set(size_t idx, int value) noexcept {
    if (idx < size_ && data_) data_[idx] = value;
}

std::unique_ptr<Buffer> make_buffer(size_t n) noexcept {
    return std::unique_ptr<Buffer>(new Buffer(n));
}

int noexcept_basic_anchor() { return 47; }

} // namespace noexcept_basic_ns
