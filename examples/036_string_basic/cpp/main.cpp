#include "string_basic.h"
#include <iostream>

int main() {
    std::cout << concat("hello, ", "world") << std::endl;
    std::cout << upper("mixed Case") << std::endl;
    std::cout << "length=" << length("hello") << std::endl;
    return 0;
}
