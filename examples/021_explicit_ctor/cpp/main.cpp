#include "explicit_ctor.h"

int main() {
    using namespace explicit_ctor_ns;
    Distance a(100.5);
    Distance b(2, 50);   // 2m 50cm = 2.5m
    a.add(b);
    std::cout << "a.meters=" << a.meters() << std::endl;

    Wrapper w("config", 3);
    std::cout << "w tag=" << w.tag() << " level=" << w.level() << std::endl;
    return 0;
}
