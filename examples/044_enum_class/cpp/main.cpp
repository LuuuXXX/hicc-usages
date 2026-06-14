#include "enum_class.h"
#include <iostream>

int main() {
    using namespace enum_class_ns;
    Color c = Color::Green;
    std::cout << "color_to_int(Green) = " << color_to_int(c) << std::endl;
    std::cout << "color_name(Green) = " << color_name(c) << std::endl;

    Color c2 = color_from_int(2);
    std::cout << "color_from_int(2) name = " << color_name(c2) << std::endl;

    Status s = Status::Pending;
    std::cout << "status_to_int(Pending) = " << status_to_int(s) << std::endl;

    Light l(Color::Red);
    std::cout << "light.current name = " << color_name(l.current()) << std::endl;
    std::cout << "light.brightness = " << l.brightness() << std::endl;
    l.set(Color::Blue);
    std::cout << "after set light.brightness = " << l.brightness() << std::endl;

    return 0;
}
