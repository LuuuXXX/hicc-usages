#include "hicc_usages/virtual_diamond.h"
#include <cassert>
#include <cstring>
#include <iostream>
int main() {
    using namespace hicc_usages::virtual_diamond;
    IODevice* io = IODevice::create(42);
    assert(std::strcmp(io->device_type(), "IODevice") == 0);
    assert(io->priority() == 10);
    assert(io->read() == 42);
    assert(io->write(100) == 100);
    assert(io->state() == 100);
    assert(io->read() == 100);
    IODevice::free(io);
    std::cout << "[virtual_diamond] C++ test OK" << std::endl;
    return 0;
}
