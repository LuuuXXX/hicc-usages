#include "class_static.h"

int main() {
    using namespace class_static_ns;
    std::cout << "before: alive=" << Counter::alive()
              << " next_id=" << Counter::next_id()
              << " species=" << Counter::species() << std::endl;

    {
        Counter a;
        a.inc(); a.inc();
        Counter b;
        b.inc();
        std::cout << "a id=" << a.id() << " count=" << a.count() << std::endl;
        std::cout << "b id=" << b.id() << " count=" << b.count() << std::endl;
        std::cout << "alive=" << Counter::alive() << std::endl;
    }
    std::cout << "after: alive=" << Counter::alive() << std::endl;
    return 0;
}
