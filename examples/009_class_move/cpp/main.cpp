#include "class_move.h"
#include <utility>

int main() {
    using namespace class_move_ns;
    Holder a(3, "A");
    a += 10;
    std::cout << "a size=" << a.size() << " first=" << a.first() << " tag=" << a.tag() << std::endl;

    Holder b = std::move(a);   // 移动构造
    std::cout << "b size=" << b.size() << " a size=" << a.size() << std::endl;

    Holder c;
    c = std::move(b);          // 移动赋值
    std::cout << "c size=" << c.size() << " b size=" << b.size() << std::endl;
    return 0;
}
