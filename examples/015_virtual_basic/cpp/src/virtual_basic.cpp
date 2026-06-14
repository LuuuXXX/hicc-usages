#include "hicc_usages/virtual_basic.h"
#include <cmath>
namespace hicc_usages::virtual_basic {
Shape* Shape::create() { return new Shape(); }
void Shape::free(Shape* self) { delete self; }
double Shape::area() const { return 0.0; }
const char* Shape::name() const { return "Shape"; }
Circle::Circle(double r) : radius_(r) {}
Circle* Circle::create(double radius) { return new Circle(radius); }
void Circle::free(Circle* self) { delete self; }
double Circle::area() const { return 3.14159265 * radius_ * radius_; }
const char* Circle::name() const { return "Circle"; }
double Circle::radius() const { return radius_; }
}
