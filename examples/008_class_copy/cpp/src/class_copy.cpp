#include "hicc_usages/class_copy.h"
#include <cstring>
namespace hicc_usages::class_copy {
Buffer::Buffer(int capacity) : capacity_(capacity), size_(0) {
    data_ = new int[capacity];
}
Buffer* Buffer::create(int capacity) { return new Buffer(capacity); }
Buffer* Buffer::clone(const Buffer* src) {
    Buffer* b = new Buffer(src->capacity_);
    b->size_ = src->size_;
    std::memcpy(b->data_, src->data_, sizeof(int) * src->size_);
    return b;
}
void Buffer::free(Buffer* self) { delete[] self->data_; delete self; }
int Buffer::capacity() const { return capacity_; }
int Buffer::size() const { return size_; }
void Buffer::append(int value) { if (size_ < capacity_) data_[size_++] = value; }
}
