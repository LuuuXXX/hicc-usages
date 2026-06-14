#include "hicc_usages/vector_basic.h"
namespace hicc_usages::vector_basic {
IntVector::IntVector() : data_() {}
IntVector::~IntVector() = default;
IntVector* IntVector::create() { return new IntVector(); }
void IntVector::free(IntVector* self) { delete self; }
void IntVector::push_back(int v) { data_.push_back(v); }
int IntVector::at(std::size_t idx) const { return data_.at(idx); }
void IntVector::pop_back() { data_.pop_back(); }
std::size_t IntVector::size() const { return data_.size(); }
int IntVector::sum() const {
    int total = 0;
    for (int v : data_) total += v;
    return total;
}
void IntVector::clear() { data_.clear(); }
}
