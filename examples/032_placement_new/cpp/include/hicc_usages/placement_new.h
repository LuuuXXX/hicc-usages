#pragma once
#include <cstddef>
#include <iostream>
#include <new>
namespace hicc_usages::placement_new {

class Buffer {
public:
    static Buffer* create(std::size_t capacity);
    static void free(Buffer* self);
    std::size_t capacity() const;
    std::size_t used() const;
    void reset();

    int place_int(int v);
    int get_int(std::size_t idx) const;
    double place_double(double v);
    double get_double(std::size_t idx) const;
private:
    explicit Buffer(std::size_t cap);
    ~Buffer();
    char* storage_;
    std::size_t capacity_;
    std::size_t int_count_;
    std::size_t double_count_;
};

}  // namespace hicc_usages::placement_new
