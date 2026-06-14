#pragma once

// Single inheritance. Rust side treats the derived class as an independent
// import_class! — hicc has no notion of C++ inheritance. Public methods
// inherited from the base are exposed by re-declaring them on the derived
// (or by exposing them through forwarding methods).

class Shape {
public:
    virtual ~Shape() = default;
    virtual int area() const = 0;
    int id() const { return id_; }
protected:
    explicit Shape(int id) : id_(id) {}
    int id_;
};

class Square : public Shape {
public:
    explicit Square(int side) : Shape(1), side_(side) {}
    int area() const override { return side_ * side_; }
    int side() const { return side_; }
private:
    int side_;
};

Square* square_new(int side);
void     square_free(Square* s);
