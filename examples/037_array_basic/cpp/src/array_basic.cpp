#include "hicc_usages/array_basic.h"
#include <algorithm>
#include <stdexcept>
namespace hicc_usages::array_basic {
FixedArray::FixedArray() : data_{} {}
FixedArray::~FixedArray() = default;
FixedArray* FixedArray::create() { return new FixedArray(); }
void FixedArray::free(FixedArray* self) { delete self; }
void FixedArray::set(std::size_t idx, int v) {
    if (idx >= ARRAY_SIZE) throw std::out_of_range("idx");
    data_[idx] = v;
}
int FixedArray::get(std::size_t idx) const {
    if (idx >= ARRAY_SIZE) throw std::out_of_range("idx");
    return data_[idx];
}
std::size_t FixedArray::size() const { return ARRAY_SIZE; }
int FixedArray::sum() const {
    int total = 0;
    for (int v : data_) total += v;
    return total;
}
int FixedArray::max() const {
    return *std::max_element(data_.begin(), data_.end());
}
void FixedArray::fill(int v) { data_.fill(v); }
}
