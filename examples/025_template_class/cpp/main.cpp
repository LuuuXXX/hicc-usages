#include "template_class.h"
#include <iostream>

int main() {
    BoxT<int> bi(42);
    BoxT<double> bd(3.14);
    std::cout << "int box: " << bi.get() << std::endl;
    std::cout << "double box: " << bd.get() << std::endl;
    return 0;
}
