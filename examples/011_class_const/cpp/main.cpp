#include "class_const.h"
#include <iostream>

int main() {
    Vec2 a(3.0, 4.0);
    Vec2 b(1.0, 0.0);
    std::cout << "a.magnitude() = " << a.magnitude() << std::endl;
    std::cout << "a.dot(b)      = " << a.dot(b) << std::endl;
    return 0;
}
