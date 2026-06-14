#pragma once
#include <string>
#include <iostream>
#include <cmath>

namespace virtual_basic_ns {

class Shape {
public:
    Shape(const std::string& n) : name_(n) {}
    virtual ~Shape() = default;

    const std::string& name() const { return name_; }
    virtual float area() const { return 0.0f; }
    virtual float perimeter() const { return 0.0f; }
    virtual std::string describe() const {
        return name_ + " area=" + std::to_string(area());
    }
protected:
    std::string name_;
};

class Rectangle : public Shape {
public:
    Rectangle(float w, float h) : Shape("rect"), width_(w), height_(h) {}
    float area() const override { return width_ * height_; }
    float perimeter() const override { return 2.0f * (width_ + height_); }
private:
    float width_, height_;
};

class Ellipse : public Shape {
public:
    Ellipse(float a, float b) : Shape("ellipse"), a_(a), b_(b) {}
    float area() const override { return 3.14159f * a_ * b_; }
    float perimeter() const override {
        return 2.0f * 3.14159f * std::sqrt((a_*a_ + b_*b_) / 2.0f);
    }
private:
    float a_, b_;
};

} // namespace virtual_basic_ns
