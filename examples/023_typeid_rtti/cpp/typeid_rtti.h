#pragma once

#include <string>
#include <typeinfo>

// RTTI: typeid() and dynamic type info. Exposed via named accessor functions
// that return type name strings (raw typeid() results are not FFI-safe).

class Shape {
public:
    virtual ~Shape() = default;
    virtual int area() const = 0;
};

class Circle : public Shape {
public:
    explicit Circle(int r) : r_(r) {}
    int area() const override { return 3 * r_ * r_; }  // pi ≈ 3
private:
    int r_;
};

class Triangle : public Shape {
public:
    Triangle(int b, int h) : b_(b), h_(h) {}
    int area() const override { return b_ * h_ / 2; }
private:
    int b_, h_;
};

// Named RTTI accessors — Rust binds these.
const char* type_name_of(const Shape* s);   // calls typeid(*s).name()
const char* static_type_name_circle();
const char* static_type_name_triangle();

Circle*   circle_new(int r);
Triangle* triangle_new(int b, int h);
void      shape_free_circle(Circle* c);
void      shape_free_triangle(Triangle* t);
