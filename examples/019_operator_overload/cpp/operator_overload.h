#pragma once
#include <iostream>

namespace operator_overload_ns {

class Vec2 {
public:
    Vec2(float x = 0, float y = 0) : x_(x), y_(y) {}

    float x() const { return x_; }
    float y() const { return y_; }

    // 二元算术 operator
    Vec2 operator+(const Vec2& o) const { return Vec2(x_ + o.x_, y_ + o.y_); }
    Vec2 operator-(const Vec2& o) const { return Vec2(x_ - o.x_, y_ - o.y_); }
    Vec2 operator*(float s) const { return Vec2(x_ * s, y_ * s); }

    // 复合赋值
    Vec2& operator+=(const Vec2& o) { x_ += o.x_; y_ += o.y_; return *this; }

    // 比较
    bool operator==(const Vec2& o) const { return x_ == o.x_ && y_ == o.y_; }

    // 下标
    float operator[](int i) const { return i == 0 ? x_ : y_; }

    // 一元负号
    Vec2 operator-() const { return Vec2(-x_, -y_); }

private:
    float x_, y_;
};

inline std::ostream& operator<<(std::ostream& os, const Vec2& v) {
    return os << "(" << v.x() << "," << v.y() << ")";
}

} // namespace operator_overload_ns
