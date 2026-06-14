#include "typeid_rtti.h"
#include <iostream>

int main() {
    Circle* c = circle_new(2);
    Triangle* t = triangle_new(4, 6);
    Shape* s1 = c;
    Shape* s2 = t;
    std::cout << "circle    type = " << type_name_of(s1) << std::endl;
    std::cout << "triangle  type = " << type_name_of(s2) << std::endl;
    std::cout << "static circle type = " << static_type_name_circle() << std::endl;
    shape_free_circle(c);
    shape_free_triangle(t);
    return 0;
}
