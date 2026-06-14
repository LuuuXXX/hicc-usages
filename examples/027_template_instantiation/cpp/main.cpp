#include "template_instantiation.h"

int main() {
    using namespace template_instantiation_ns;
    Pair<int> p1(10, 20);
    std::cout << "p1 first=" << p1.first() << " second=" << p1.second() << " sum=" << p1.sum() << std::endl;
    p1.swap();
    std::cout << "after swap first=" << p1.first() << std::endl;

    Pair<std::string> p2("hello", "world");
    std::cout << "p2 sum=" << p2.sum() << std::endl;
    return 0;
}
