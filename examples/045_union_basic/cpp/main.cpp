#include "union_basic.h"
#include <iostream>

int main() {
    using namespace union_basic_ns;

    Value vi = make_value_int(42);
    Value vf = make_value_float(3.14f);
    Value vl = make_value_long(1234567890L);
    std::cout << "value_as_int = " << value_as_int(vi) << std::endl;
    std::cout << "value_as_float = " << value_as_float(vf) << std::endl;
    std::cout << "value_as_long = " << value_as_long(vl) << std::endl;

    Box b1 = make_box_int(7);
    Box b2 = make_box_float(2.5f);
    Box b3 = make_box_long(99L);
    std::cout << "b1 " << b1.describe() << std::endl;
    std::cout << "b2 " << b2.describe() << std::endl;
    std::cout << "b3 " << b3.describe() << std::endl;

    b1.set_float(1.5f);
    std::cout << "after set_float b1 " << b1.describe() << std::endl;

    Box b4 = b2;  // 拷贝构造
    std::cout << "b4 (copy of b2) " << b4.describe() << std::endl;
    return 0;
}
