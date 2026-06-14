#include "hicc_usages/raii_pattern.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::raii_pattern;
    {
        File* f = File::create(5);
        assert(f->valid());
        f->read(100);
        f->write(50);
        File::free(f);  // 析构时 close() 被调用
    }
    std::cout << "[raii_pattern] C++ test OK" << std::endl;
    return 0;
}
