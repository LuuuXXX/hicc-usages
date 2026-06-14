#include "unique_ptr.h"
#include <iostream>

int main() {
    auto w = make_widget(42);
    std::cout << "widget value = " << w->value() << std::endl;
    return 0;
}
