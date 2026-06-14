#pragma once

// Const methods are the bread-and-butter of read-only API. hicc maps
// `&self` in Rust to `const` member methods.

class Vec2 {
public:
    Vec2(double x, double y) : x_(x), y_(y) {}
    double x() const { return x_; }
    double y() const { return y_; }
    double magnitude() const;
    double dot(const Vec2& other) const { return x_ * other.x_ + y_ * other.y_; }
private:
    double x_, y_;
};
