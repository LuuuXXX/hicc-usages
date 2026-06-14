#include "virtual_basic.h"
#include <iostream>

int main() {
    Dog* d = dog_new("Rex");
    Animal* a = d;  // upcast — virtual dispatch through base pointer
    std::cout << a->sound() << " says " << a->name() << std::endl;
    dog_free(d);
    return 0;
}
