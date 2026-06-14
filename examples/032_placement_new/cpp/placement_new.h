#pragma once
#include <new>
#include <iostream>
#include <cstring>

namespace placement_new_ns {

class Buffer {
public:
    Buffer(size_t sz) : size_(sz) {
        mem_ = new char[sz];
        std::cout << "Buffer(" << sz << ") alloc" << std::endl;
    }
    ~Buffer() {
        delete[] mem_;
        std::cout << "~Buffer() free" << std::endl;
    }
    void* raw() { return mem_; }
    size_t size() const { return size_; }
private:
    char* mem_;
    size_t size_;
};

class Payload {
public:
    Payload(int v) : value_(v) {
        std::cout << "Payload(" << v << ") ctor (placement)" << std::endl;
    }
    ~Payload() {
        std::cout << "~Payload(" << value_ << ") dtor" << std::endl;
    }
    int value() const { return value_; }
    void set(int v) { value_ = v; }
private:
    int value_;
};

// 在已有 buffer 上构造 Payload（placement new）
Payload* place_payload(Buffer& buf, int value);
// 在任意 raw 内存上构造 Payload（FFI 友好版本）
Payload* place_payload_raw(void* raw, int value);
// 显式调析构（不释放内存）
void destroy_payload(Payload* p);

} // namespace placement_new_ns
