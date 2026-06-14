#pragma once

#include <vector>
#include <cstddef>

// std::vector<int> as a member of a class. We expose factory + size + index.
// Rust binds it via hicc-std's vector<Pod<T>> alias.

class IntVector {
public:
    IntVector() = default;
    void push(int v) { data_.push_back(v); }
    std::size_t size() const { return data_.size(); }
    int at(std::size_t i) const { return data_[i]; }
    // Direct accessor (returns a const reference to internal vector — used
    // when wrapping via hicc-std on the Rust side).
    const std::vector<int>& raw() const { return data_; }
private:
    std::vector<int> data_;
};

IntVector* int_vec_new();
void       int_vec_free(IntVector* v);
