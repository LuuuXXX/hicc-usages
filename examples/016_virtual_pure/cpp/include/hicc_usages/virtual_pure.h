#pragma once
namespace hicc_usages::virtual_pure {
class Shape {
public:
    virtual double area() const = 0;
    virtual const char* name() const = 0;
    virtual ~Shape() = default;
};
class Square : public Shape {
public:
    static Square* create(double side);
    static void free(Square* self);
    double area() const override;
    const char* name() const override;
    double side() const;
private:
    explicit Square(double s);
    double side_;
};
class Triangle : public Shape {
public:
    static Triangle* create(double base, double height);
    static void free(Triangle* self);
    double area() const override;
    const char* name() const override;
private:
    Triangle(double b, double h);
    double base_, height_;
};
}
