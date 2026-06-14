#include "hicc_usages/hello_world.h"

namespace hicc_usages::hello_world {

void hello() {
    std::cout << "Hello, World!" << std::endl;
}

Greeter* Greeter::create() {
    return new Greeter();
}

void Greeter::free(Greeter* self) {
    delete self;
}

void Greeter::greet() const {
    std::cout << "Greetings from Greeter!" << std::endl;
}

}  // namespace hicc_usages::hello_world
