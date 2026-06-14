#include "hicc_usages/inheritance_multiple.h"
namespace hicc_usages::inheritance_multiple {
int Drawable::draw_calls_ = 0;
int Printable::print_calls_ = 0;
int Drawable::draw_calls() { return draw_calls_; }
const char* Drawable::shape_name() const { return "Drawable"; }
int Printable::print_calls() { return print_calls_; }
const char* Printable::printable_text() const { return "Printable"; }
Shape* Shape::create() { return new Shape(); }
void Shape::free(Shape* self) { delete self; }
const char* Shape::shape_name() const { return "Shape"; }
const char* Shape::printable_text() const { return "Shape printable"; }
}
