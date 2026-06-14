#pragma once

// Operator overloading. hicc can't bind operator+ directly, so we provide
// named wrapper functions (vec2_add / vec2_sub) that invoke the operators.

class Vec2 {
public:
    Vec2(int x, int y) : x_(x), y_(y) {}
    int x() const { return x_; }
    int y() const { return y_; }

    Vec2 operator+(const Vec2& o) const { return Vec2(x_ + o.x_, y_ + o.y_); }
    Vec2 operator-(const Vec2& o) const { return Vec2(x_ - o.x_, y_ - o.y_); }
    bool operator==(const Vec2& o) const { return x_ == o.x_ && y_ == o.y_; }

private:
    int x_, y_;
};

// Named wrappers (invoke operators internally) — these are what Rust binds.
Vec2 vec2_add(const Vec2& a, const Vec2& b);
Vec2 vec2_sub(const Vec2& a, const Vec2& b);
bool vec2_eq(const Vec2& a, const Vec2& b);

Vec2* vec2_new(int x, int y);
void  vec2_free(Vec2* v);
