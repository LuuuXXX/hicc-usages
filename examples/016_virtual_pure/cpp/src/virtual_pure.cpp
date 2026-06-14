#include "hicc_usages/virtual_pure.h"
namespace hicc_usages::virtual_pure {
Square::Square(double s) : side_(s) {}
Square* Square::create(double side) { return new Square(side); }
void Square::free(Square* self) { delete self; }
double Square::area() const { return side_ * side_; }
const char* Square::name() const { return "Square"; }
double Square::side() const { return side_; }
Triangle::Triangle(double b, double h) : base_(b), height_(h) {}
Triangle* Triangle::create(double base, double height) { return new Triangle(base, height); }
void Triangle::free(Triangle* self) { delete self; }
double Triangle::area() const { return 0.5 * base_ * height_; }
const char* Triangle::name() const { return "Triangle"; }
}
