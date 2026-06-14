#pragma once
#include <cstddef>
#include <iostream>
#include <array>
namespace hicc_usages::array_basic {

constexpr std::size_t ARRAY_SIZE = 8;

class FixedArray {
public:
    static FixedArray* create();
    static void free(FixedArray* self);
    void set(std::size_t idx, int v);
    int get(std::size_t idx) const;
    std::size_t size() const;
    int sum() const;
    int max() const;
    void fill(int v);
private:
    FixedArray();
    ~FixedArray();
    std::array<int, ARRAY_SIZE> data_;
};

}  // namespace hicc_usages::array_basic
