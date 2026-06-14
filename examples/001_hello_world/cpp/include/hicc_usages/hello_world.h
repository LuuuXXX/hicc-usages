#pragma once

#include <cstddef>
#include <iostream>

namespace hicc_usages::hello_world {

void hello();

class Greeter {
public:
    static Greeter* create();
    static void free(Greeter* self);
    void greet() const;
};

}  // namespace hicc_usages::hello_world
