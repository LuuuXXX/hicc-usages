#include "hicc_usages/hello_world.h"
#include <cassert>

int main() {
    using namespace hicc_usages::hello_world;

    hello();

    Greeter* g = Greeter::create();
    assert(g != nullptr);
    g->greet();
    Greeter::free(g);

    std::cout << "[hello_world] C++ test OK" << std::endl;
    return 0;
}
