#include "summary.h"
#include <iostream>

int main() {
    std::cout << "calc_version() = " << calc_version() << std::endl;
    Calculator* c = calc_new(100);
    std::cout << "describe: " << c->describe() << std::endl;
    std::cout << "apply(Add,2,3) = " << c->apply(OpKind::Add, 2, 3) << std::endl;
    try {
        c->apply(OpKind::Div, 1, 0);
    } catch (const std::exception& e) {
        std::cout << "caught: " << e.what() << std::endl;
    }
    calc_free(c);
    return 0;
}
