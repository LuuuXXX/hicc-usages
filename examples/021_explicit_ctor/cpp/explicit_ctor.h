#pragma once
#include <string>
#include <iostream>

namespace explicit_ctor_ns {

// explicit 阻止隐式转换；FFI 端透明
class Distance {
public:
    explicit Distance(double meters) : meters_(meters) {}
    explicit Distance(int whole_meters, int cm)
        : meters_(whole_meters + cm / 100.0) {}

    double meters() const { return meters_; }
    void add(const Distance& other) { meters_ += other.meters_; }

private:
    double meters_;
};

class Wrapper {
public:
    // explicit + 多参数
    explicit Wrapper(const std::string& tag, int level)
        : tag_(tag), level_(level) {}

    const std::string& tag() const { return tag_; }
    int level() const { return level_; }

private:
    std::string tag_;
    int level_;
};

} // namespace explicit_ctor_ns
