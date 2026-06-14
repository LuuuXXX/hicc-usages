#pragma once
namespace hicc_usages::class_copy {
class Buffer {
public:
    static Buffer* create(int capacity);
    static Buffer* clone(const Buffer* src);
    static void free(Buffer* self);
    int capacity() const;
    int size() const;
    void append(int value);
private:
    Buffer(int capacity);
    Buffer(const Buffer&) = default;
    int* data_;
    int capacity_;
    int size_ = 0;
};
}
