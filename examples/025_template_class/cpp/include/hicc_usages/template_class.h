#pragma once
#include <cstddef>
#include <iostream>
#include <vector>
namespace hicc_usages::template_class {

template<typename T>
class Stack {
public:
    void push(T v) { data_.push_back(v); }
    T pop() { T v = data_.back(); data_.pop_back(); return v; }
    std::size_t size() const { return data_.size(); }
    T top() const { return data_.back(); }
private:
    std::vector<T> data_;
};

}
