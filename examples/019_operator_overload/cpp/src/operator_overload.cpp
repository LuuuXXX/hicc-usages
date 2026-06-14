#include "hicc_usages/operator_overload.h"
namespace hicc_usages::operator_overload {
Vec2::Vec2(int x, int y) : x_(x), y_(y) {}
Vec2* Vec2::create(int x, int y) { return new Vec2(x, y); }
void Vec2::free(Vec2* self) { delete self; }
int Vec2::get_x() const { return x_; }
int Vec2::get_y() const { return y_; }
int Vec2::dot(const Vec2& other) const { return x_ * other.x_ + y_ * other.y_; }
Vec2 Vec2::operator+(const Vec2& other) const { return Vec2(x_ + other.x_, y_ + other.y_); }
Vec2 Vec2::operator-(const Vec2& other) const { return Vec2(x_ - other.x_, y_ - other.y_); }
Vec2 Vec2::operator*(int factor) const { return Vec2(x_ * factor, y_ * factor); }
bool Vec2::operator==(const Vec2& other) const { return x_ == other.x_ && y_ == other.y_; }
}
