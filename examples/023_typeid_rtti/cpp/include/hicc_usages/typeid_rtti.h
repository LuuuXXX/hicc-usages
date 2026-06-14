#pragma once
namespace hicc_usages::typeid_rtti {
class Shape {
public:
    virtual ~Shape() = default;
    virtual const char* type_name() const;
    virtual int id() const;
};
class Circle : public Shape {
public:
    static Circle* create(double r);
    static void free(Circle* self);
    const char* type_name() const override;
    int id() const override;
    double radius() const;
    bool is_circle() const;
private:
    explicit Circle(double r);
    double r_;
};
class Square : public Shape {
public:
    static Square* create(double s);
    static void free(Square* self);
    const char* type_name() const override;
    int id() const override;
    double side() const;
    bool is_square() const;
private:
    explicit Square(double s);
    double s_;
};
// 用 dynamic_cast 包装的 RTTI 函数
bool is_circle_shape(const Shape* s);
bool is_square_shape(const Shape* s);
}
