#pragma once
#include <string>
#include <iostream>

namespace default_args_ns {

inline int greet(const std::string& name, int times = 1, const std::string& suffix = "!") {
    for (int i = 0; i < times; ++i) {
        std::cout << "Hello, " << name << suffix << std::endl;
    }
    return times;
}

inline int compute(int a, int b = 10, int c = 100) {
    return a + b + c;
}

} // namespace default_args_ns
