#include "class_static.h"
#include <iostream>

int main() {
    std::cout << "live_count=" << Registry::live_count() << std::endl;
    {
        Registry a, b, c;
        std::cout << "live_count=" << Registry::live_count()
                  << " next_id=" << Registry::next_id() << std::endl;
    }
    std::cout << "live_count=" << Registry::live_count() << std::endl;
    return 0;
}
