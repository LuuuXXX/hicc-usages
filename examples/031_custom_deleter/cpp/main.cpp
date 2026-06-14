#include "custom_deleter.h"
#include <iostream>

int main() {
    auto f = open_file(42);
    std::cout << "fd = " << f->fd << std::endl;
    return 0;  // FileClose runs at scope exit
}
