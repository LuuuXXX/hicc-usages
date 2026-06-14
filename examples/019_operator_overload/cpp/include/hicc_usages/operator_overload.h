#pragma once
namespace hicc_usages::operator_overload {
class Vec2 {
public:
    static Vec2* create(int x, int y);
    static void free(Vec2* self);
    int get_x() const;
    int get_y() const;
    int dot(const Vec2& other) const;
    Vec2 operator+(const Vec2& other) const;
    Vec2 operator-(const Vec2& other) const;
    Vec2 operator*(int factor) const;
    bool operator==(const Vec2& other) const;
private:
    Vec2(int x, int y);
    int x_, y_;
};
}
