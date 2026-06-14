#include "string_basic.h"

int main() {
    using namespace string_basic_ns;
    auto g = greet("world");
    std::cout << "greet=" << g << std::endl;
    std::cout << "upper=" << to_upper(g) << std::endl;
    std::cout << "concat=" << concat("foo", "bar") << std::endl;
    std::cout << "length=" << string_length(g) << std::endl;
    std::cout << "contains 'world'=" << contains_substring(g, "world") << std::endl;
    std::cout << "contains 'xyz'=" << contains_substring(g, "xyz") << std::endl;
    return 0;
}
