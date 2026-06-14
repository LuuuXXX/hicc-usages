#pragma once
#include <string>
#include <iostream>

namespace class_const_ns {

// 演示 const 成员函数与 const 正确性
class Temperature {
public:
    Temperature(float v) : value_(v), unit_("C") {}
    Temperature(float v, const std::string& u) : value_(v), unit_(u) {}

    // const 成员函数：只读访问
    float value() const { return value_; }
    const std::string& unit() const { return unit_; }
    float to_fahrenheit() const {
        if (unit_ == "C") return value_ * 9.0f / 5.0f + 32.0f;
        return value_;
    }
    std::string describe() const {
        return std::to_string(value_) + " " + unit_;
    }

    // 非 const 成员函数：修改
    void set_value(float v) { value_ = v; }
    void convert_to(const std::string& new_unit) {
        if (unit_ == "C" && new_unit == "F") {
            value_ = value_ * 9.0f / 5.0f + 32.0f;
            unit_ = "F";
        } else if (unit_ == "F" && new_unit == "C") {
            value_ = (value_ - 32.0f) * 5.0f / 9.0f;
            unit_ = "C";
        }
    }

private:
    float value_;
    std::string unit_;
};

} // namespace class_const_ns
