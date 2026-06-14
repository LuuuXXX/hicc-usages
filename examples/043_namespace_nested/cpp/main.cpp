#include "namespace_nested.h"
#include <iostream>

int main() {
    using namespace n1::n2::n3;
    auto foo = make_foo(42);
    std::cout << "foo.value() = " << foo->value() << std::endl;
    std::cout << "foo.describe() = " << foo->describe() << std::endl;
    foo->set_value(100);
    std::cout << "after set foo.value() = " << foo->value() << std::endl;
    std::cout << "compute(5) = " << compute(5) << std::endl;

    auto bar = n1::inner::make_bar("hello");
    std::cout << "bar.name() = " << bar->name() << std::endl;
    bar->rename("world");
    std::cout << "after rename bar.name() = " << bar->name() << std::endl;

    std::cout << "add(3,4) = " << outer::deep::deeper::add(3, 4) << std::endl;
    std::cout << "triple(7) = " << outer::deep::deeper::triple(7) << std::endl;
    return 0;
}
