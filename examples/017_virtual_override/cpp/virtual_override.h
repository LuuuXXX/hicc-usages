#pragma once
#include <string>
#include <iostream>

namespace virtual_override_ns {

class Shape {
public:
    Shape(const std::string& n) : name_(n) {}
    virtual ~Shape() = default;

    const std::string& name() const { return name_; }
    virtual int sides() const { return 0; }
    virtual std::string describe() const { return name_ + "/sides=" + std::to_string(sides()); }
protected:
    std::string name_;
};

class Triangle : public Shape {
public:
    Triangle(const std::string& n) : Shape(n) {}
    int sides() const override { return 3; }
};

class Pentagon : public Shape {
public:
    Pentagon(const std::string& n) : Shape(n) {}
    int sides() const override { return 5; }
    std::string describe() const override { return name_ + "(pentagon)"; }
};

} // namespace virtual_override_ns
