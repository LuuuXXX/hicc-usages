#pragma once
namespace hicc_usages::virtual_basic {
class Shape {
public:
    static Shape* create();
    static void free(Shape* self);
    virtual double area() const;
    virtual const char* name() const;
    virtual ~Shape() = default;
};
class Circle : public Shape {
public:
    static Circle* create(double radius);
    static void free(Circle* self);
    double area() const override;
    const char* name() const override;
    double radius() const;
private:
    explicit Circle(double r);
    double radius_;
};
}
