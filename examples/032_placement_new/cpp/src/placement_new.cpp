#include "hicc_usages/placement_new.h"
#include <cstdlib>
#include <cstring>
namespace hicc_usages::placement_new {

Buffer::Buffer(std::size_t cap)
    : storage_(static_cast<char*>(std::malloc(cap))),
      capacity_(cap), int_count_(0), double_count_(0) {}
Buffer::~Buffer() { std::free(storage_); }

Buffer* Buffer::create(std::size_t capacity) { return new Buffer(capacity); }
void Buffer::free(Buffer* self) { delete self; }
std::size_t Buffer::capacity() const { return capacity_; }
std::size_t Buffer::used() const {
    return int_count_ * sizeof(int) + double_count_ * sizeof(double);
}
void Buffer::reset() { int_count_ = 0; double_count_ = 0; }

int Buffer::place_int(int v) {
    std::size_t offset = used();
    if (offset + sizeof(int) > capacity_) return -1;
    new (storage_ + offset) int(v);
    ++int_count_;
    return static_cast<int>(int_count_ - 1);
}
int Buffer::get_int(std::size_t idx) const {
    if (idx >= int_count_) return -1;
    int* p = reinterpret_cast<int*>(storage_);
    return p[idx];
}
double Buffer::place_double(double v) {
    std::size_t offset = used();
    if (offset + sizeof(double) > capacity_) return -1.0;
    new (storage_ + offset) double(v);
    ++double_count_;
    return static_cast<double>(double_count_ - 1);
}
double Buffer::get_double(std::size_t idx) const {
    if (idx >= double_count_) return -1.0;
    std::size_t int_bytes = int_count_ * sizeof(int);
    double* p = reinterpret_cast<double*>(storage_ + int_bytes);
    return p[idx];
}

}
