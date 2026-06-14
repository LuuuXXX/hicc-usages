#include "hicc_usages/enum_class.h"
#include <cassert>
#include <cstring>
#include <iostream>
int main() {
    using namespace hicc_usages::enum_class;
    assert(color_to_int(Color::Red) == 0);
    assert(color_to_int(Color::Blue) == 2);
    assert(int_to_color(1) == Color::Green);
    assert(strcmp(color_name(Color::Yellow), "Yellow") == 0);
    assert(is_primary(Color::Red));
    assert(!is_primary(Color::Yellow));

    assert(direction_opposite(0) == 1);  // North <-> South
    assert(direction_opposite(2) == 3);  // East <-> West

    Pixel* p = Pixel::create(10, 20, Color::Red);
    assert(p->x() == 10);
    assert(p->y() == 20);
    assert(p->color() == Color::Red);
    assert(p->is_warm());
    p->set_color(Color::Blue);
    assert(!p->is_warm());
    Pixel::free(p);
    std::cout << "[enum_class] C++ test OK" << std::endl;
    return 0;
}
