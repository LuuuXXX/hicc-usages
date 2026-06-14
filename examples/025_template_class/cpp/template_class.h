#pragma once
#include <string>
#include <vector>
#include <iostream>

namespace template_class_ns {

template <typename T>
class Stack {
public:
    Stack() = default;

    void push(const T& v) { data_.push_back(v); }
    void pop() { if (!data_.empty()) data_.pop_back(); }
    T top() const { return data_.empty() ? T() : data_.back(); }
    size_t size() const { return data_.size(); }
    bool empty() const { return data_.empty(); }

private:
    std::vector<T> data_;
};

// 显式实例化：Stack<int> 与 Stack<std::string>
template class Stack<int>;
template class Stack<std::string>;

} // namespace template_class_ns
