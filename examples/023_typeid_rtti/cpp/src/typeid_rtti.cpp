#include "hicc_usages/typeid_rtti.h"
#include <typeinfo>
namespace hicc_usages::typeid_rtti {
const char* Shape::type_name() const { return typeid(*this).name(); }
int Shape::id() const { return 0; }
Circle::Circle(double r) : r_(r) {}
Circle* Circle::create(double r) { return new Circle(r); }
void Circle::free(Circle* self) { delete self; }
const char* Circle::type_name() const { return "Circle"; }
int Circle::id() const { return 1; }
double Circle::radius() const { return r_; }
bool Circle::is_circle() const { return true; }
Square::Square(double s) : s_(s) {}
Square* Square::create(double s) { return new Square(s); }
void Square::free(Square* self) { delete self; }
const char* Square::type_name() const { return "Square"; }
int Square::id() const { return 2; }
double Square::side() const { return s_; }
bool Square::is_square() const { return true; }
bool is_circle_shape(const Shape* s) { return dynamic_cast<const Circle*>(s) != nullptr; }
bool is_square_shape(const Shape* s) { return dynamic_cast<const Square*>(s) != nullptr; }
}
