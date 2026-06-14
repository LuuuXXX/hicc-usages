#include "functional_bind.h"
#include <iostream>

int main() {
    std::cout << "add_bound_10(5) = " << add_bound_10(5) << std::endl;
    std::cout << "mul_bound_3(7) = " << mul_bound_3(7) << std::endl;
    std::cout << "sub_bind_first(10,3) = " << sub_bind_first(10, 3) << std::endl;

    BindPoint* p = bind_point_new(42, 100);
    std::cout << "point_x_plus_offset(p, 8) = "
              << point_x_plus_offset(p, 8) << std::endl;
    bind_point_free(p);
    return 0;
}
