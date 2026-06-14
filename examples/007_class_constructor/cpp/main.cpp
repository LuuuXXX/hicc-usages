#include "class_constructor.h"
#include <iostream>

int main() {
    Point* p = point_new(3, -4);
    std::cout << "x=" << p->get_x() << " y=" << p->get_y()
              << " |manhattan|=" << p->manhattan() << std::endl;
    point_free(p);
    return 0;
}
