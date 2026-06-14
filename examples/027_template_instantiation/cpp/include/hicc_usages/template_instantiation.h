#pragma once
#include <cstddef>
#include <iostream>
namespace hicc_usages::template_instantiation {

template<typename T>
class Container {
public:
    Container() : value_(T{}) {}
    void set(T v) { value_ = v; }
    T get() const { return value_; }
    T doubled() const { return value_ + value_; }
private:
    T value_;
};

}
