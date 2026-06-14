#include "inheritance_single.h"

int main() {
    using namespace inheritance_single_ns;
    Dog d("Rex");
    Cat c("Mimi");
    std::cout << d.name() << " says " << d.sound() << " legs=" << d.legs() << std::endl;
    std::cout << c.name() << " says " << c.sound() << " legs=" << c.legs() << std::endl;

    Animal* a = &d;
    std::cout << "via base: " << a->name() << " " << a->sound() << std::endl;
    return 0;
}
