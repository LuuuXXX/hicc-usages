#include "virtual_basic.h"

int main() {
    using namespace virtual_basic_ns;
    Rectangle r(3.0f, 4.0f);
    Ellipse e(5.0f, 2.0f);
    std::cout << r.describe() << " perimeter=" << r.perimeter() << std::endl;
    std::cout << e.describe() << " perimeter=" << e.perimeter() << std::endl;

    Shape* shapes[] = {&r, &e};
    for (auto* s : shapes) {
        std::cout << s->name() << " area=" << s->area() << std::endl;
    }
    return 0;
}
