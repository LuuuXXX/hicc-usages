#pragma once

#include <array>
#include <cstddef>

// std::array<int, N>: fixed-size. Expose accessor methods.

template <std::size_t N>
class IntArrayN {
public:
    IntArrayN() { data_.fill(0); }
    void set(std::size_t i, int v) { data_[i] = v; }
    int  get(std::size_t i) const { return data_[i]; }
    std::size_t size() const { return N; }
    int sum() const {
        int s = 0;
        for (int x : data_) s += x;
        return s;
    }
private:
    std::array<int, N> data_;
};

// Concrete typedef for N=4.
using IntArray4 = IntArrayN<4>;
IntArray4* int_array4_new();
void        int_array4_free(IntArray4* a);
