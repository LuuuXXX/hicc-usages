#include "shared_ptr.h"

int main() {
    using namespace shared_ptr_ns;
    auto c1 = make_counter(10);
    std::cout << "c1 value=" << c1->value() << " use_count=" << use_count(c1) << std::endl;

    auto c2 = clone_counter(c1);
    std::cout << "after clone use_count=" << use_count(c1) << std::endl;

    c2->increment();
    std::cout << "after c2->increment c1 value=" << c1->value() << std::endl;
    return 0;
}
