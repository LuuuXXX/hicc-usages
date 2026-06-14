#include "typeid_rtti.h"

const char* type_name_of(const Shape* s) {
    return typeid(*s).name();
}

const char* static_type_name_circle()   { return typeid(Circle).name(); }
const char* static_type_name_triangle() { return typeid(Triangle).name(); }

Circle*   circle_new(int r)        { return new Circle(r); }
Triangle* triangle_new(int b, int h) { return new Triangle(b, h); }
void      shape_free_circle(Circle* c)   { delete c; }
void      shape_free_triangle(Triangle* t) { delete t; }
