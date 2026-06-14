#include "hello_world.h"

int main() {
    hello_world_ns::hello_world();
    std::cout << "answer = " << hello_world_ns::answer() << std::endl;
    return 0;
}
