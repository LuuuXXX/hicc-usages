#include "class_copy.h"

int main() {
    class_copy_ns::Buffer a(10, "A");
    class_copy_ns::Buffer b = a;            // 拷贝构造
    class_copy_ns::Buffer c = std::move(a); // 移动构造
    class_copy_ns::Buffer d;
    d = b;                                  // 拷贝赋值
    std::cout << "b size=" << b.size() << " tag=" << b.tag() << std::endl;
    std::cout << "c size=" << c.size() << " tag=" << c.tag() << std::endl;
    std::cout << "d size=" << d.size() << " tag=" << d.tag() << std::endl;
    return 0;
}
