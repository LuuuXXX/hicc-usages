#pragma once
#include <string>
#include <iostream>

namespace template_instantiation_ns {

template <typename T>
class Pair {
public:
    Pair(T first, T second) : first_(first), second_(second) {}
    T first() const { return first_; }
    T second() const { return second_; }
    T sum() const { return first_ + second_; }
    void swap() {
        T tmp = first_;
        first_ = second_;
        second_ = tmp;
    }
private:
    T first_;
    T second_;
};

// 显式实例化声明
extern template class Pair<int>;
extern template class Pair<std::string>;

} // namespace template_instantiation_ns
