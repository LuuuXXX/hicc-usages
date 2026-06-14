#include "class_copy.h"

int main() {
    class_copy_ns::Buffer a(10, "A");
    class_copy_ns::Buffer b = a;            // copy ctor
    class_copy_ns::Buffer c = std::move(a); // move ctor
    class_copy_ns::Buffer d;
    d = b;                                  // copy assign
    std::cout << "b size=" << b.size() << " tag=" << b.tag() << std::endl;
    std::cout << "c size=" << c.size() << " tag=" << c.tag() << std::endl;
    std::cout << "d size=" << d.size() << " tag=" << d.tag() << std::endl;
    return 0;
}
