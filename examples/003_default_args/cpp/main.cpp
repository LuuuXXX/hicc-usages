#include "default_args.h"
#include <iostream>

int main() {
    std::cout << "add(5)      = " << add(5) << std::endl;       // default b=10
    std::cout << "add(5, 20)  = " << add(5, 20) << std::endl;
    return 0;
}
