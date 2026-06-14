#pragma once
#include <string>
#include <iostream>

namespace inheritance_multiple_ns {

class Drawable {
public:
    virtual ~Drawable() = default;
    virtual void draw() const = 0;
    virtual std::string shape() const { return "Drawable"; }
};

class Serializable {
public:
    virtual ~Serializable() = default;
    virtual std::string serialize() const = 0;
    virtual int bytes() const { return 0; }
};

class Circle : public Drawable, public Serializable {
public:
    Circle(float r) : radius_(r) {}
    void draw() const override {
        std::cout << "Drawing Circle r=" << radius_ << std::endl;
    }
    std::string shape() const override { return "Circle"; }
    std::string serialize() const override {
        return "Circle{r=" + std::to_string(radius_) + "}";
    }
    int bytes() const override { return sizeof(float) + 8; }
    float radius() const { return radius_; }
private:
    float radius_;
};

class Square : public Drawable, public Serializable {
public:
    Square(float side) : side_(side) {}
    void draw() const override {
        std::cout << "Drawing Square side=" << side_ << std::endl;
    }
    std::string shape() const override { return "Square"; }
    std::string serialize() const override {
        return "Square{s=" + std::to_string(side_) + "}";
    }
    float side() const { return side_; }
private:
    float side_;
};

} // namespace inheritance_multiple_ns
