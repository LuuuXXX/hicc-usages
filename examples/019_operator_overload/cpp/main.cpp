#include "operator_overload.h"
#include <iostream>

int main() {
    Vec2* a = vec2_new(1, 2);
    Vec2* b = vec2_new(3, 4);
    Vec2 c = vec2_add(*a, *b);
    std::cout << "a+b = (" << c.x() << "," << c.y() << ")" << std::endl;
    std::cout << "a==b? " << (vec2_eq(*a, *b) ? "yes" : "no") << std::endl;
    vec2_free(a);
    vec2_free(b);
    return 0;
}
