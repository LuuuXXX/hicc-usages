#include "inheritance_single.h"
#include <iostream>

int main() {
    Square* s = square_new(4);
    std::cout << "area=" << s->area() << " side=" << s->side() << " id=" << s->id() << std::endl;
    square_free(s);
    return 0;
}
