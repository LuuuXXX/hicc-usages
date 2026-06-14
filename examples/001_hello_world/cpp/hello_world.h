#pragma once
#include <iostream>

namespace hello_world_ns {

inline void hello_world() {
    std::cout << "Hello, World! [from C++ hello_world]" << std::endl;
}

inline int answer() {
    return 42;
}

} // namespace hello_world_ns
