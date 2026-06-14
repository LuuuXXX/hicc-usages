#include "hicc_usages/custom_deleter.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::custom_deleter;
    FileManager* m = FileManager::create();
    m->open(3);
    m->open(4);
    m->open(5);
    assert(m->open_count() == 3);
    assert(m->close(4));
    assert(m->open_count() == 2);
    assert(!m->close(99));
    FileManager::free(m);
    std::cout << "[custom_deleter] C++ test OK" << std::endl;
    return 0;
}
