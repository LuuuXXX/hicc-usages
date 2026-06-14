#include "enum_class.h"
#include <iostream>

int main() {
    Color c = int_to_color(1);
    std::cout << "color_name(int_to_color(1)) = " << color_name(c) << std::endl;
    std::cout << "to_int_red() = " << to_int_red() << std::endl;
    std::cout << "color_name_for_int(2) = " << color_name_for_int(2) << std::endl;
    return 0;
}
